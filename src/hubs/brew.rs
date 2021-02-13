use reqwest::header;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomebrewRecipe {
    pub name: String,
    pub desc: String,
    pub homepage: String,

    // pub urls: String,
    // pub versions: String,
}

pub fn get_projects() -> Vec<crate::projects::project::SoftwareProject> {
    vec![]
}
