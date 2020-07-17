// See `man flatpak-manifest` for the flatpak manifest specs.
struct FlatpakManifest {
    name: String,

    // A string defining the application id.
    app_id: String,

    // The branch to use when exporting the application. If this is unset the defaults come from the default-branch option.

    // This key overrides both the default-branch key, and the --default-branch commandline option.
    // Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
    // to use the default-branch key instead, because you can then override the default using
    // --default-branch when building for instance a test build.
    branch: String,

    // The default branch to use when exporting the application. Defaults to master.
    // This key can be overridden by the --default-branch commandline option.
    default_branch: String,

    // The collection ID of the repository, defaults to being unset. Setting a globally unique collection
    // ID allows the apps in the repository to be shared over
    // peer to peer systems without needing further configuration. If building in an existing repository,
    // the collection ID must match the existing configured collection ID for that repository.
    collection_id: String,

    // The name of the runtime that the application uses.
    runtime: String,

    // The version of the runtime that the application uses, defaults to master.
    runtime_version: String,

    // The name of the development runtime that the application builds with.
    sdk: String,

    // Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
    var: String,

    // Use this file as the base metadata file when finishing.
    metadata: String,

    // Build a new runtime instead of an application.
    build_runtime: bool,

    // Build an extension.
    build_extension: bool,

    // Start with the files from the specified application.
    // This can be used to create applications that extend another application.
    base: String,

    // Use this specific version of the application specified in base. If unspecified, this uses the value specified in branch
    base_version: String,

    // Install these extra extensions from the base application when
    // initializing the application directory.
    base_extensions: [String;5],

    // Inherit these extra extensions points from the base application or sdk when finishing the build.
    inherit_extensions: [String;5],

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    inherit_sdk_extensions: [String;5],

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    build_options: [String;5],

    // Add these tags to the metadata file.
    tags: [String;5],

    // modules = [Module]
}
