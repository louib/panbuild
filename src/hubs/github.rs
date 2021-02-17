use std::env;

use futures::executor::block_on;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub fn get_org_repos(repo_name: &str) -> Vec<String> {
    let mut next_url = format!("https://api.github.com/orgs/{}/repos?type=all&per_page=100", repo_name);
    let mut repos = vec![];

    let mut headers = header::HeaderMap::new();
    if let Ok(token) = env::var("PB_GITHUB_TOKEN") {
        headers.insert("Authorization", header::HeaderValue::from_str(&token).unwrap());
    } else {
        log::warn!("No GitHub API token located at PB_GITHUB_TOKEN. We will get rate limited faster.");
    }

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    while next_url.len() != 0 {
        // TODO make this really asynchronous with async/await.
        let mut response = match client.get(&next_url).send() {
            Ok(r) => r,
            Err(e) => return repos,
        };

        if response.status().as_u16() == 204 {
            return repos;
        }

        // let response_content = response.text().unwrap();
        let response_headers = response.headers();

        let link_header = match &response_headers.get("link") {
            Some(h) => h.to_str().unwrap(),
            None => return repos,
        };
        next_url = crate::utils::get_next_page_url(link_header).to_string();
    }

    repos
}

// See https://docs.github.com/en/rest/reference/repos
#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {
    id: String,
    name: String,
    full_name: String,
    description: String,
    fork: bool,
    is_template: bool,
    archived: bool,
    disabled: bool,
    topics: Vec<String>,
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
        project.keywords = self.topics;
        project
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {}
