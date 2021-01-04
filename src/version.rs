use serde::{Deserialize, Serialize};

// See https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md
// and https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
#[derive(Serialize, Deserialize, Default)]
pub struct SemanticVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_release: String,
    pub build: String,
}
impl SemanticVersion {
    // From https://semver.org/:
    // <valid semver> ::= <version core>
    //                  | <version core> "-" <pre-release>
    //                  | <version core> "+" <build>
    //                  | <version core> "-" <pre-release> "+" <build>
    pub fn parse(version: &String) -> Option<SemanticVersion> {
        let parts: Vec<&str> = version.split('-').collect();
        let mut pre_release = "".to_string();
        let mut build = "".to_string();

        let version_core = parts[0].trim().to_string();

        if parts.len() >= 2 {
            let right_part: Vec<&str> = parts[1].split('+').collect();
            pre_release = right_part[0].to_string();
        }

        // let right_part: Vec<&str> = version.split('+').collect();
        // build = right_part[0];

        let version_parts: Vec<&str> = version_core.split('.').collect();
        if version_parts.len() != 3 {
            eprintln!("Invalid semantic version {}.", version_core);
            return None;
        }

        let major: i32 = match version_parts[0].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid major version {}", version_parts[0]);
                return None;
            },
        };
        let minor: i32 = match version_parts[1].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid minor version {}", version_parts[1]);
                return None;
            },
        };
        let patch: i32 = match version_parts[2].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid patch version {}", version_parts[2]);
                return None;
            },
        };

        Some(SemanticVersion {
            major: major,
            minor: minor,
            patch: patch,
            pre_release: pre_release,
            build: build,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_invalid_strings() {
        assert!(SemanticVersion::parse(&"not a version".to_string()).is_none());
        assert!(SemanticVersion::parse(&"fdsfsdfd.2.3".to_string()).is_none());
        assert!(SemanticVersion::parse(&"1.fsdfsd.3".to_string()).is_none());
        assert!(SemanticVersion::parse(&"1.2.fsdfsd".to_string()).is_none());
    }

    #[test]
    pub fn test_parse_simple_version() {
        let mut sem_ver = SemanticVersion::parse(&"1.2.3".to_string());
        assert!(sem_ver.is_some());
        let sem_ver = sem_ver.unwrap();
        assert_eq!(sem_ver.major, 1);
        assert_eq!(sem_ver.minor, 2);
        assert_eq!(sem_ver.patch, 3);
    }

    #[test]
    pub fn test_parse_version_with_release() {
        let mut sem_ver = SemanticVersion::parse(&"1.2.3-alpha".to_string());
        assert!(sem_ver.is_some());
        let sem_ver = sem_ver.unwrap();
        assert_eq!(sem_ver.major, 1);
        assert_eq!(sem_ver.minor, 2);
        assert_eq!(sem_ver.patch, 3);
        assert_eq!(sem_ver.pre_release, "alpha".to_string());
    }
}
