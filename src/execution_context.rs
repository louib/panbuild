pub const DEFAULT_SOURCE_TYPE: &str = "snap";
pub const DEFAULT_DESTINATION_TYPE: &str = "flatpak";

pub struct ExecutionContext {
    pub source_filename: String,
    pub source_type: String,
    pub destination_type: String,
    pub data_dir: String,
    pub content: String,
    pub manifest: crate::manifests::manifest::AbstractManifest,
    pub flatpak_manifest: Option<crate::manifests::flatpak::FlatpakManifest>,
    // pub debian_manifest: Option<crate::manifests::debian::DebianManifest>,
    pub snap_manifest: Option<crate::manifests::snap::SnapcraftManifest>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        return ExecutionContext {
            source_filename: "".to_string(),
            source_type: DEFAULT_SOURCE_TYPE.to_string(),
            destination_type: DEFAULT_DESTINATION_TYPE.to_string(),
            data_dir: "".to_string(),
            content: "".to_string(),
            manifest: crate::manifests::manifest::AbstractManifest::default(),
            flatpak_manifest: None,
            // debian_manifest: None,
            snap_manifest: None,
        };
    }
}
