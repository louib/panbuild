use std::env;

use futures::executor::block_on;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub fn get_org_repos(repo_name: &str) -> Vec<String> {
    let repos_url = format!("https://api.github.com/orgs/{}/repos?type=all&per_page=100", repo_name);
    let mut repos = vec![];

    let mut headers = header::HeaderMap::new();
    if let Ok(token) = env::var("PB_GITHUB_TOKEN") {
        headers.insert("Authorization", header::HeaderValue::from_str(&token).unwrap());
    }

    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    repos
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHub {}
