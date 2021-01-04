use serde::{Deserialize, Serialize};

// See https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md
// and https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field
#[derive(Serialize, Deserialize, Default)]
pub struct SemanticVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    // TODO there's a string after the patch...
}
impl SemanticVersion {
    // From https://semver.org/:
    // <valid semver> ::= <version core>
    //                  | <version core> "-" <pre-release>
    //                  | <version core> "+" <build>
    //                  | <version core> "-" <pre-release> "+" <build>
    pub fn parse(version: &String) -> SemanticVersion {
        let mut patch: String = String::from("");
        let mut minor: String = String::from("");
        let mut major: String = String::from("");
        SemanticVersion {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}
