use serde::{Deserialize, Serialize};

/// Main representation for a software hub. A software hub is any online service
/// that offers a web and/or vcs access to software projects. Software projects
/// can be discovered through API calls, manifest file parsing or yet other
/// means.
#[derive(Debug, Serialize, Deserialize)]
pub enum SoftwareHub {
    GitHub(crate::hubs::github::GitHub),
}
