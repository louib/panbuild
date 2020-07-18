struct DebianManifest {
    source: String,
    section: String,
    priority: String,

    // Format is name <email@address.com>
    maintainer: String,

    build_depends: Vec<String>,
}
