use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const DEFAULT_SOURCE_TYPE: &str = "flatpak";

pub struct ExecutionContext {
    pub source_filename: String,
    pub source_type: String,
    pub data_dir: String,
    pub content: String,
    pub manifest: crate::manifests::manifest::AbstractManifest,
}
impl Default for ExecutionContext {
    fn default() -> Self {
        return ExecutionContext {
            source_filename: "".to_string(),
            source_type: DEFAULT_SOURCE_TYPE.to_string(),
            data_dir: "".to_string(),
            content: "".to_string(),
            manifest: crate::manifests::manifest::AbstractManifest::default(),
        };
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct PanbuildConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_workspace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_build: Option<String>,

    pub envs: BTreeMap<String, String>,
}

pub fn write_config(config: &PanbuildConfig) -> Result<PanbuildConfig, String> {
    Ok(PanbuildConfig::default())
}

pub fn read_config() -> Result<PanbuildConfig, String> {
    Ok(PanbuildConfig::default())
}
