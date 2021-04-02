use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SoftwareDeveloper {
    pub name: String,
    pub aliases: Vec<String>,
    pub emails: Vec<String>,
}
