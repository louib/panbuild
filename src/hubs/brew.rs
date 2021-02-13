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
    // All the formulae for macOS
    // wget https://formulae.brew.sh/api/formula.json

    // All the formulae for Linux
    // wget https://formulae.brew.sh/api/formula-linux.json

    // All the casks
    // wget https://formulae.brew.sh/api/cask.json
    vec![]
}
