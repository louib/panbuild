pub const DEFAULT_SOURCE_TYPE: &str = "snap";
pub const DEFAULT_DESTINATION_TYPE: &str = "flatpak";

pub struct ExecutionContext {
    pub source_filename: String,
    pub source_type: String,
    pub destination_type: String,
    pub content: String,
    pub abstract_manifest: crate::manifests::abstract_manifest::AbstractManifest,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        return ExecutionContext {
            source_filename: "".to_string(),
            source_type: DEFAULT_SOURCE_TYPE.to_string(),
            destination_type: DEFAULT_DESTINATION_TYPE.to_string(),
            content: "".to_string(),
            abstract_manifest: crate::manifests::abstract_manifest::AbstractManifest::default(),
        };
    }
}
