use serde::{Deserialize, Serialize};

// The format is defined in https://www.python.org/dev/peps/pep-0518/
pub struct PyProjectManifest {

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
