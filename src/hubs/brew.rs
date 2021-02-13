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

    // pub urls: String,
    pub versions: HomebrewRecipeVersions,
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

pub fn get_projects() -> Vec<crate::projects::project::SoftwareProject> {
    // All the formulae for macOS
    let all_mac_formulas_url = "https://formulae.brew.sh/api/formula.json";
    // All the formulae for Linux
    let all_linux_formulas_url = "https://formulae.brew.sh/api/formula-linux.json";
    // All casks
    let all_casks_formulas_url = "https://formulae.brew.sh/api/cask.json";

    let client = reqwest::blocking::Client::builder().build().unwrap();

    // TODO make this really asynchronous with async/await.
    let mut response = match client.get(all_mac_formulas_url).send() {
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

    vec![]
}
