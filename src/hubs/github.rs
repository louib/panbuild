use std::env;

use futures::executor::block_on;
use reqwest::header;
use serde::{Deserialize, Serialize};

fn get_next_page_url(link_header: &str) -> &str {
    for link in link_header.split(",") {
        let mut link_parts = link.split(";");
        let url = link_parts.nth(0).unwrap();
        let rel = link_parts.nth(1).unwrap();
        if !rel.contains("rel=\"next\"") {
            continue;
        }
        let mut next_page_url = url.trim();
        next_page_url = &next_page_url[1..next_page_url.len() - 1];
        return next_page_url;
    }
    ""
}

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

    while (next_url.len() != 0) {
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
        next_url = get_next_page_url(link_header).to_string();
    }

    repos
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {}
