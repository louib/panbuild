use reqwest::header;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomebrewRecipe {
    pub name: String,
    pub full_name: String,
    pub tap: String,
    pub aliases: Vec<String>,
    pub license: Option<String>,
    pub desc: String,
    pub homepage: String,

    pub disabled: bool,
    pub deprecated: bool,
    pub outdated: bool,
    pub pinned: bool,

    pub urls: HomebrewRecipeUrls,
    pub versions: HomebrewRecipeVersions,
}
impl HomebrewRecipe {
    pub fn to_software_project(self) -> crate::projects::SoftwareProject {
        let mut project = crate::projects::SoftwareProject::default();
        // We filter out http:// urls for now, but could try to convert to https in the future.
        if self.urls.stable.url.ends_with(".git") && self.urls.stable.url.starts_with("https") {
            project.id = crate::utils::repo_url_to_reverse_dns(&self.urls.stable.url);
            project.vcs_urls.push(self.urls.stable.url);
        }
        project
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomebrewRecipeVersions {
    pub stable: String,
    pub bottle: bool,
    pub head: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomebrewRecipeUrls {
    pub stable: HomebrewRecipeUrl,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomebrewRecipeUrl {
    pub url: String,
    pub tag: Option<String>,
    pub revision: Option<String>,
}

pub fn get_and_add_recipes(db: &mut crate::db::Database) {
    // All the formulae for macOS
    for project in get_projects("https://formulae.brew.sh/api/formula.json") {
        db.add_project(project);
    }

    // All the formulae for Linux
    for project in get_projects("https://formulae.brew.sh/api/formula-linux.json") {
        db.add_project(project);
    }

    // There are also the cask formulae, but they have a different format.
    // https://formulae.brew.sh/api/cask.json
}

pub fn get_projects(formulae_url: &str) -> Vec<crate::projects::SoftwareProject> {
    let mut projects: Vec<crate::projects::SoftwareProject> = vec![];

    let client = reqwest::blocking::Client::builder().build().unwrap();

    // TODO make this really asynchronous with async/await.
    let mut response = match client.get(formulae_url).send() {
        Ok(r) => r,
        Err(e) => return vec![],
    };

    let brew_recipes: Vec<HomebrewRecipe> = match serde_json::from_str(&response.text().unwrap()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Could not parse brew recipes {}.", e);
            return vec![];
        }
    };

    for brew_recipe in brew_recipes {
        let project = brew_recipe.to_software_project();
        if project.id.len() == 0 {
            continue;
        }
        log::info!("Adding project {} from brew recipe.", project.name);
        projects.push(project);
    }

    projects
}
