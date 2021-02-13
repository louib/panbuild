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
    // FIXME there is actually an object defined in there.
    pub forked_from_project: Option<bool>,
}
impl GitLabProject {
    pub fn to_software_project(self) -> crate::projects::project::SoftwareProject {
        let mut project = crate::projects::project::SoftwareProject::default();
        project.id = crate::utils::repo_url_to_reverse_dns(&self.http_url_to_repo);
        project.name = self.name;
        if let Some(branch) = self.default_branch {
            project.default_branch = branch;
        }
        project.description = self.description.unwrap_or("".to_string());
        project.vcs_urls.push(self.http_url_to_repo);
        project.keywords = self.tag_list;
        project
    }
}

pub struct PagedResponse {
    pub next_page_url: Option<String>,
    pub results: Vec<crate::projects::project::SoftwareProject>,
}

pub struct PagedRequest {
    pub next_page_url: Option<String>,
    pub domain: String,
}

pub fn get_and_add_repos(domain: &str, db: &mut crate::db::Database) {
    log::info!("Getting all projects from GitLab instance at {}.", domain);
    let mut paged_response = get_repos(PagedRequest {
        domain: domain.to_string(),
        next_page_url: None,
    });
    let mut projects = paged_response.results;
    while projects.len() > 0 {
        for project in projects {
            println!("Adding project {}.", &project.name);
            db.add_project(project);
        }

        if paged_response.next_page_url.is_none() {
            break;
        }

        paged_response = get_repos(PagedRequest {
            domain: domain.to_string(),
            next_page_url: paged_response.next_page_url,
        });
        projects = paged_response.results;
    }
}

pub fn get_repos(request: PagedRequest) -> PagedResponse {
    let mut next_url = format!("https://{}/api/v4/projects?per_page=100", request.domain);
    if let Some(url) = request.next_page_url {
        next_url = url;
    }

    let mut projects: Vec<crate::projects::project::SoftwareProject> = vec![];
    let default_response = PagedResponse {
        results: vec![],
        next_page_url: None,
    };

    let mut headers = header::HeaderMap::new();
    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    println!("Getting GitLab projects page at {}.", next_url);
    // TODO make this really asynchronous with async/await.
    let mut response = match client.get(&next_url).send() {
        Ok(r) => r,
        Err(e) => return default_response,
    };

    if response.status().as_u16() == 204 {
        return default_response;
    }

    let response_headers = response.headers();

    let link_header = match &response_headers.get("link") {
        Some(h) => h.to_str().unwrap(),
        None => return default_response,
    };
    next_url = crate::utils::get_next_page_url(link_header).to_string();

    let gitlab_projects: Vec<GitLabProject> = match serde_yaml::from_str(&response.text().unwrap()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not parse gitlab projects {}.", e);
            return default_response;
        }
    };
    for gitlab_project in gitlab_projects {
        log::info!("Adding GitLab project {}.", gitlab_project.name);
        projects.push(gitlab_project.to_software_project());
    }

    // FIXME next_url should already be an option!
    if next_url.len() == 0 {
        return PagedResponse {
            results: projects,
            next_page_url: None,
        };
    }

    PagedResponse {
        results: projects,
        next_page_url: Some(next_url),
    }
}
