use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// The format is defined in https://www.python.org/dev/peps/pep-0518/
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PyProjectManifest {
    pub project: PyProjectProject,
    pub dependencies: BTreeMap<String, PyProjectDependency>,
}
impl PyProjectManifest {
    pub fn get_type(&self) -> &str {
        return "pyproject";
    }

    pub fn file_path_matches(path: &str) -> bool {
        if path.to_lowercase().ends_with("pyproject.toml") {
            return true;
        }
        false
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PyProjectProject {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub readme: String,
    // python version that this package targets.
    pub python: String,
    pub homepage: String,
    pub repository: String,
    pub documentation: String,
    pub keywords: Vec<String>,
    pub classifiers: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PyProjectDependency {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(PyProjectManifest::file_path_matches("pyproject.toml"));
        assert!(PyProjectManifest::file_path_matches("./pyproject.toml"));
        assert!(PyProjectManifest::file_path_matches("./path/to/the/pyproject.toml"));
        assert!(!PyProjectManifest::file_path_matches("com.example.appName.yaml"));
        assert!(!PyProjectManifest::file_path_matches(""));
        assert!(!PyProjectManifest::file_path_matches("/////////////"));
    }
}
