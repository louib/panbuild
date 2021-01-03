use std::collections::BTreeMap;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

// Make that more robust maybe?
pub const DEFAULT_CACHE_DIR: &str = ".panbuild/";

pub struct ExecutionContext {
    pub data_dir: String,
    pub manifest: crate::manifests::manifest::AbstractManifest,
}
impl Default for ExecutionContext {
    fn default() -> Self {
        return ExecutionContext {
            data_dir: "".to_string(),
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

    pub workspaces: BTreeMap<String, String>,
}

pub fn write_config(config: &PanbuildConfig) -> Result<PanbuildConfig, String> {
    let cache_dir = path::Path::new(DEFAULT_CACHE_DIR);
    if !cache_dir.is_dir() {
        match fs::create_dir(cache_dir) {
            Ok(_) => {}
            Err(e) => return Err(format!("Could not create cache dir at {}", DEFAULT_CACHE_DIR)),
        };
    }

    let config_content = match serde_yaml::to_string(&config) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to dump the config {}", e)),
    };

    let config_path = DEFAULT_CACHE_DIR.to_owned() + "config.yaml";
    let config_path = path::Path::new(&config_path);
    match fs::write(config_path, config_content) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to write the config file at {}: {}", config_path.to_str().unwrap_or(""), e)),
    };

    read_config()
}

pub fn read_config() -> Result<PanbuildConfig, String> {
    // Make that more robust maybe?
    let config_path = DEFAULT_CACHE_DIR.to_owned() + "config.yaml";
    let config_path = path::Path::new(&config_path);
    let config_content = match fs::read_to_string(config_path) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to read the config file at {}", config_path.to_str().unwrap_or(""))),
    };

    let config: PanbuildConfig = match serde_yaml::from_str(&config_content) {
        Ok(m) => m,
        Err(e) => return Err(format!("Failed to parse the config file at {}: {}.", config_path.to_str().unwrap_or(""), e)),
    };
    Ok(config)
}

pub fn read_or_init_config() -> Result<PanbuildConfig, String> {
    match read_config() {
        Ok(config) => Ok(config),
        Err(_) => match write_config(&PanbuildConfig::default()) {
            Ok(c) => return Ok(c),
            Err(e) => return Err(e),
        },
    }
}
