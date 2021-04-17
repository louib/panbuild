use std::env;

use reqwest::header;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLab {}

// GitLab API described here
// https://docs.gitlab.com/ee/api/projects.html
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabProject {
    pub id: String,
    pub name: String,
    pub name_with_namespace: String,
    pub created_at: String,
    pub last_activity_at: String,
    pub forks_count: i32,
    pub star_count: i32,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub readme_url: String,
    pub tag_list: Vec<String>,
    // From the API doc:
    // If the project is a fork, and you provide a valid token to authenticate,
    // the forked_from_project field appears in the response.
    pub forked_from_project: Option<GitLabParentProject>,
}
impl GitLabProject {
    pub fn to_software_project(self) -> panbuild::projects::SoftwareProject {
        let mut project = panbuild::projects::SoftwareProject::default();
        project.id = panbuild::utils::repo_url_to_reverse_dns(&self.http_url_to_repo);
        project.name = self.name;
        project.default_branch = self.default_branch;
        project.description = self.description.unwrap_or("".to_string());
        project.vcs_urls.push(self.http_url_to_repo);
        project.keywords = self.tag_list;
        project
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabParentProject {
    pub id: String,
    pub name: String,
}

pub fn get_and_add_repos(domain: &str, token_env_var_name: &str, db: &mut panbuild::db::Database) {
    log::info!("Getting all projects from GitLab instance at {}.", domain);
    let mut request = panbuild::utils::PagedRequest {
        domain: domain.to_string(),
        token: None,
        next_page_url: None,
    };
    if let Ok(token) = env::var(token_env_var_name) {
        // See https://docs.gitlab.com/ee/api/#oauth2-tokens
        // for documentation on OAuth authentication.
        request.token = Some(token);
    } else {
        log::warn!("No GitLab API token located at {} for instance at {}. Aborting.", token_env_var_name, domain);
        return;
    }
    let mut paged_response = get_repos(request);

    let mut projects = paged_response.results;
    while projects.len() > 0 {
        for project in projects {
            log::debug!("Adding project {}.", &project.name);
            db.add_project(project);
        }

        if paged_response.next_page_url.is_none() {
            break;
        }

        paged_response = get_repos(panbuild::utils::PagedRequest {
            domain: domain.to_string(),
            token: paged_response.token,
            next_page_url: paged_response.next_page_url,
        });
        projects = paged_response.results;
    }
}

pub fn get_repos(request: panbuild::utils::PagedRequest) -> panbuild::utils::PagedResponse {
    let mut current_url = format!("https://{}/api/v4/projects?per_page=100&simple=false", request.domain);
    if let Some(url) = request.next_page_url {
        current_url = url;
    }

    let mut projects: Vec<panbuild::projects::SoftwareProject> = vec![];
    let default_response = panbuild::utils::PagedResponse {
        results: vec![],
        token: None,
        next_page_url: None,
    };

    let mut headers = header::HeaderMap::new();
    let auth_header_value = format!("Bearer {}", request.token.as_ref().unwrap());
    let auth_header = header::HeaderValue::from_str(&auth_header_value.to_string()).unwrap();
    headers.insert("Authorization", auth_header);
    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    log::info!("Getting GitLab projects page at {}.", current_url);
    // TODO make this really asynchronous with async/await.
    let mut response = match client.get(&current_url).send() {
        Ok(r) => r,
        Err(e) => {
            log::error!("Could not fetch GitLab url {}: {}.", current_url, e);
            return default_response;
        }
    };

    if response.status().as_u16() == 204 {
        return default_response;
    }

    let response_headers = response.headers();

    let link_header = match &response_headers.get("link") {
        Some(h) => h.to_str().unwrap(),
        None => "",
    };
    let next_page_url = panbuild::utils::get_next_page_url(link_header);

    let gitlab_projects: Vec<GitLabProject> = match serde_yaml::from_str(&response.text().unwrap()) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Could not parse gitlab projects {}.", e);
            return default_response;
        }
    };
    for gitlab_project in gitlab_projects {
        if let Some(parent_project) = gitlab_project.forked_from_project {
            log::debug!("Skipping forked project {}.", &gitlab_project.name);
            continue;
        }
        log::debug!("Adding GitLab project {}.", gitlab_project.name);
        projects.push(gitlab_project.to_software_project());
    }

    panbuild::utils::PagedResponse {
        results: projects,
        token: request.token,
        next_page_url: next_page_url,
    }
}
