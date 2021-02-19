use std::env;

use futures::executor::block_on;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub fn get_org_repos(org_name: &str) -> Vec<crate::projects::SoftwareProject> {
    let mut paged_response = get_repos(crate::utils::PagedRequest {
        domain: "".to_string(),
        next_page_url: Some(format!("https://api.github.com/orgs/{}/repos?type=all&per_page=100", org_name)),
    });
    let mut all_projects = vec![];
    let mut projects = paged_response.results;
    while projects.len() > 0 {
        for project in projects {
            println!("Adding project {}.", &project.name);
            all_projects.push(project);
        }

        if paged_response.next_page_url.is_none() {
            break;
        }

        paged_response = get_repos(crate::utils::PagedRequest {
            domain: "".to_string(),
            next_page_url: paged_response.next_page_url,
        });
        projects = paged_response.results;
    }
    all_projects
}

pub fn get_repos(request: crate::utils::PagedRequest) -> crate::utils::PagedResponse {
    // By default, we get all the repos.
    let mut current_url = format!("https://api.github.com/repos?type=all&per_page=100");
    if let Some(url) = request.next_page_url {
        current_url = url;
    }

    let mut projects: Vec<crate::projects::SoftwareProject> = vec![];
    let default_response = crate::utils::PagedResponse {
        results: vec![],
        next_page_url: None,
    };

    let mut headers = header::HeaderMap::new();
    // User agent is required when using the GitHub API.
    // See https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
    headers.insert("User-Agent", header::HeaderValue::from_str("panbuild").unwrap());
    if let Ok(token) = env::var("PB_GITHUB_TOKEN") {
        headers.insert("Authorization", header::HeaderValue::from_str(&token).unwrap());
    } else {
        log::warn!("No GitHub API token located at PB_GITHUB_TOKEN. We will get rate limited faster.");
    }

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    println!("Getting GitHub projects page at {}.", current_url);
    // TODO make this really asynchronous with async/await.
    let response = match client.get(&current_url).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Could not fetch GitHub url {}: {}.", current_url, e);
            return default_response;
        },
    };

    if response.status().as_u16() == 204 {
        return default_response;
    }

    let link_header = match &response.headers().get("link") {
        Some(h) => h.to_str().unwrap(),
        None => "",
    };
    let next_page_url = crate::utils::get_next_page_url(link_header);

    let github_repos: Vec<GitHubRepo> = match serde_yaml::from_str(&response.text().unwrap()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Could not parse GitHub repos {}.", e);
            return default_response;
        }
    };
    for github_project in github_repos {
        log::debug!("Adding GitHub repo {}.", github_project.name);
        projects.push(github_project.to_software_project());
    }

    crate::utils::PagedResponse {
        results: projects,
        next_page_url: next_page_url,
    }
}

// See https://docs.github.com/en/rest/reference/repos
#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {
    id: String,
    name: String,
    full_name: String,
    description: String,
    fork: bool,
    is_template: Option<bool>,
    archived: bool,
    disabled: bool,
    topics: Option<Vec<String>>,
    clone_url: String,
    git_url: String,
    homepage: String,
    forks_count: i64,
    stargazers_count: i64,
    watchers_count: i64,
    size: i64,
    default_branch: String,
}
impl GitHubRepo {
    pub fn to_software_project(self) -> crate::projects::SoftwareProject {
        let mut project = crate::projects::SoftwareProject::default();
        project.id = crate::utils::repo_url_to_reverse_dns(&self.clone_url);
        project.name = self.name;
        project.default_branch = self.default_branch;
        project.description = self.description;
        project.vcs_urls.push(self.clone_url);
        if let Some(topics) = self.topics {
            project.keywords = topics;
        }
        project
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {}
