use std::env;

use futures::executor::block_on;
use reqwest::header;
use serde::{Deserialize, Serialize};

// See https://docs.github.com/en/rest/reference/repos
#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {
    id: String,
    name: String,
    full_name: String,
    description: String,
    fork: bool,
    is_template: Option<bool>,
    archived: Option<bool>,
    disabled: Option<bool>,
    topics: Option<Vec<String>>,
    clone_url: Option<String>,
    git_url: Option<String>,
    homepage: Option<String>,
    forks_count: Option<i64>,
    stargazers_count: Option<i64>,
    watchers_count: Option<i64>,
    size: Option<i64>,
    default_branch: Option<String>,
}
impl GitHubRepo {
    pub fn to_software_project(self) -> crate::projects::SoftwareProject {
        let mut project = crate::projects::SoftwareProject::default();
        let git_url = format!("https://github.com/{}.git", self.full_name);
        project.id = crate::utils::repo_url_to_reverse_dns(&git_url);
        project.name = self.name;
        project.default_branch = self.default_branch;
        project.description = self.description;
        project.vcs_urls.push(git_url);
        if let Some(topics) = self.topics {
            project.keywords = topics;
        }
        project
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubError {
    pub message: String,
    pub documentation_url: String,
}

pub fn get_org_repos(org_name: &str) -> Vec<crate::projects::SoftwareProject> {
    let mut paged_response = get_repos(crate::utils::PagedRequest {
        domain: "".to_string(),
        token: None,
        next_page_url: Some(format!("https://api.github.com/orgs/{}/repos?type=all&per_page=100", org_name)),
    });
    let mut all_projects = vec![];
    let mut projects = paged_response.results;
    while projects.len() > 0 {
        for project in projects {
            log::info!("Adding project {}.", &project.name);
            all_projects.push(project);
        }

        if paged_response.next_page_url.is_none() {
            break;
        }

        paged_response = get_repos(crate::utils::PagedRequest {
            domain: "".to_string(),
            token: None,
            next_page_url: paged_response.next_page_url,
        });
        projects = paged_response.results;
    }
    all_projects
}

pub fn get_and_add_repos(db: &mut crate::db::Database) {
    log::info!("Getting all projects from github.com");
    let mut request = crate::utils::PagedRequest {
        domain: "".to_string(),
        token: None,
        next_page_url: None,
    };
    let mut paged_response = get_repos(request);

    let mut projects = paged_response.results;
    while projects.len() > 0 {
        for project in projects {
            log::info!("Adding project {}.", &project.name);
            db.add_project(project);
        }

        if paged_response.next_page_url.is_none() {
            break;
        }

        paged_response = get_repos(crate::utils::PagedRequest {
            domain: "".to_string(),
            token: paged_response.token,
            next_page_url: paged_response.next_page_url,
        });
        projects = paged_response.results;
    }
}

pub fn get_repos(request: crate::utils::PagedRequest) -> crate::utils::PagedResponse {
    // By default, we get all the repos.
    let mut current_url = format!("https://api.github.com/repositories?type=all&per_page=2");
    if let Some(url) = request.next_page_url {
        current_url = url;
    }

    let mut projects: Vec<crate::projects::SoftwareProject> = vec![];
    let default_response = crate::utils::PagedResponse {
        results: vec![],
        token: None,
        next_page_url: None,
    };

    let mut headers = header::HeaderMap::new();
    // User agent is required when using the GitHub API.
    // See https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
    headers.insert("User-Agent", header::HeaderValue::from_str("panbuild").unwrap());
    headers.insert("Accept", header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap());
    if let Ok(token) = env::var("PB_GITHUB_TOKEN") {
        let auth_header_value = format!("token {}", &token);
        headers.insert("Authorization", header::HeaderValue::from_str(&auth_header_value.to_string()).unwrap());
    } else {
        log::warn!("No GitHub API token located at PB_GITHUB_TOKEN. We will get rate limited faster.");
    }

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    log::info!("Getting GitHub projects page at {}.", current_url);
    // TODO make this really asynchronous with async/await.
    let response = match client.get(&current_url).send() {
        Ok(r) => r,
        Err(e) => {
            log::error!("Could not fetch GitHub url {}: {}.", current_url, e);
            return default_response;
        }
    };

    if response.status().as_u16() == 204 {
        return default_response;
    }

    if response.status().as_u16() > 399 {
        let error_object: GitHubError = match serde_yaml::from_str(&response.text().unwrap()) {
            Ok(e) => e,
            Err(e) => {
                log::error!("Could not parse GitHub error {}.", e);
                return default_response;
            }
        };
        log::error!("Error returned by the GitHub API: {}", error_object.message);
        return default_response;
    }

    let link_header = match &response.headers().get("link") {
        Some(h) => h.to_str().unwrap(),
        None => "",
    };
    let next_page_url = crate::utils::get_next_page_url(link_header);

    let response_content = response.text().unwrap();
    let github_repos: Vec<GitHubRepo> = match serde_yaml::from_str(&response_content) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Could not parse GitHub repos {}.", e);
            return default_response;
        }
    };
    for github_project in github_repos {
        if github_project.fork {
            continue;
        }
        log::debug!("Adding GitHub repo {}.", github_project.name);
        projects.push(github_project.to_software_project());
    }

    crate::utils::PagedResponse {
        results: projects,
        token: None,
        next_page_url: next_page_url,
    }
}
