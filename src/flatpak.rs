// See `man flatpak-manifest` for the flatpak manifest specs.
// TODO The hyphens in the attribute names were replaced to underscores.
// Not sure how to manage that yet.
struct FlatpakManifest {
    name: String,

    // A string defining the application id.
    app_id: String,

    // The branch to use when exporting the application.
    // If this is unset the defaults come from the default-branch option.
    //
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

    // Use this specific version of the application specified in base.
    // If unspecified, this uses the value specified in branch
    base_version: String,

    // Install these extra extensions from the base application when
    // initializing the application directory.
    base_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk when finishing the build.
    inherit_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    inherit_sdk_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    build_options: Vec<String>,

    // Add these tags to the metadata file.
    tags: Vec<String>,

    // An array of objects specifying the modules to be built in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains a module. See below for details.
    modules: Vec<Module>,

    // This is a dictionary of extension objects.
    // The key is the name of the extension.
    // See below for details.
    add_extensions: Vec<Extension>,

    // This is a dictionary of extension objects similar to add-extensions.
    // The main difference is that the extensions are added early and are
    // available for use during the build.
    add_build_extensions: Vec<BuildExtension>,

    // An array of file patterns that should be removed at the end.
    // Patterns starting with / are taken to be full pathnames (without the /app prefix),
    // otherwise they just match the basename.
    cleanup: Vec<String>,

    // An array of commandlines that are run during the cleanup phase.
    cleanup_commands: Vec<String>,

    // Extra files to clean up in the platform.
    cleanup_platform: Vec<String>,

    // An array of commandlines that are run during the cleanup phase of the platform.
    cleanup_platform_commands: Vec<String>,

    // An array of commandlines that are run after importing the base platform,
    // but before applying the new files from the sdk. This is a good place to e.g. delete
    // things from the base that may conflict with the files added in the sdk.
    prepare_platform_commands: Vec<String>,

    // An array of arguments passed to the flatpak build-finish command.
    finish_args: Vec<String>,

    // Any desktop file with this name will be renamed to a name based on id during the cleanup phase.
    rename_desktop_file: String,

    // Any appdata file with this name will be renamed to a name based on id during the cleanup phase.
    rename_appdata_file: String,

    // Any icon with this name will be renamed to a name based on id during the cleanup phase.
    // Note that this is the icon name, not the full filenames, so it should
    // not include a filename extension.
    rename_icon: String,

    // Replace the appdata project_license field with this string.
    // This is useful as the upstream license is typically only about the application itself,
    // whereas the bundled app can contain other licenses too.
    appdata_license: String,

    // If rename-icon is set, keep a copy of the old icon file.
    copy_icon: bool,

    // This string will be prefixed to the Name key in the main application desktop file.
    desktop_file_name_prefix: String,

    // This string will be suffixed to the Name key in the main application desktop file.
    desktop_file_name_suffix: String,
}

struct Module {
}
struct Extension {
}
struct BuildExtension {
}
