use reqwest::header;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLab {}

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
    pub default_branch: String,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub readme_url: String,
    pub tag_list: Vec<String>,
}
impl GitLabProject {
    pub fn to_software_project(self) -> crate::projects::project::Project {
        let mut project = crate::projects::project::Project::default();
        project.name = self.name;
        project.description = self.description.unwrap_or("".to_string());
        project.vcs_urls.push(self.http_url_to_repo);
        project.keywords = self.tag_list;
        project
    }
}

pub fn get_repos(gitlab_domain: &str) -> Vec<crate::projects::project::Project> {
    let mut next_url = format!("https://{}/api/v4/projects?per_page=100", gitlab_domain);
    let mut projects: Vec<crate::projects::project::Project> = vec![];

    let mut headers = header::HeaderMap::new();
    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();

    while next_url.len() != 0 {
        println!("Getting GitLab projects page at {}.", next_url);
        // TODO make this really asynchronous with async/await.
        let mut response = match client.get(&next_url).send() {
            Ok(r) => r,
            Err(e) => return projects,
        };

        if response.status().as_u16() == 204 {
            return projects;
        }

        // let response_content = response.text().unwrap();
        let response_headers = response.headers();

        let link_header = match &response_headers.get("link") {
            Some(h) => h.to_str().unwrap(),
            None => return projects,
        };
        next_url = crate::utils::get_next_page_url(link_header).to_string();

        let gitlab_projects: Vec<GitLabProject> = match serde_yaml::from_str(&response.text().unwrap()) {
            Ok(p) => p,
            Err(e) => continue,
        };
        for gitlab_project in gitlab_projects {
            log::info!("Adding GitLab project {}.", gitlab_project.name);
            projects.push(gitlab_project.to_software_project());
        }
    }

    projects
}
