use std::collections::BTreeMap;

extern crate yaml_rust;
extern crate linked_hash_map;

use linked_hash_map::{LinkedHashMap};
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};


// Other choices are org.gnome.Platform and org.kde.Platform
const DEFAULT_RUNTIME: &str = "org.freedesktop.Platform";
// Other choices are org.gnome.Sdk and org.kde.Sdk
const DEFAULT_SDK: &str = "org.freedesktop.Sdk";

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

    // An array of strings specifying the modules to be built in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains a module. See below for details.
    modules: Vec<String>,

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

// Module fields.
// Each module specifies a source that has to be separately built and installed.
// It contains the build options and a list of sources to download and extract before
// building.
//
// Modules can be nested, in order to turn related modules on and off with a single key.
//
// These are the properties that are accepted:
// The name of the module, used in e.g. build logs. The name is also
// used for constructing filenames and commandline arguments, therefore using spaces or '/' in
// this string is a bad idea.
const module_name: &str = "name";

// If true, skip this module
// (boolean)
const module_disabled: &str = "disabled";

// An array of objects defining sources that will be downloaded and extracted in order. String members in the array are interpreted as the name of a separate
// json or yaml file that contains sources. See below for details.
// (array of objects or strings)
const sources: &str = "sources";

// An array of options that will be passed to configure
// (array of strings)
const config_opts: &str = "";

// An array of arguments that will be passed to make
// (array of strings)
const make_args: &str = "";

// An array of arguments that will be passed to make install
// (array of strings)
const make_install_args: &str = "";

// If true, remove the configure script before starting build
// (boolean)
const rm_configure: &str = "";

// Ignore the existence of an autogen script
// (boolean)
const no_autogen: &str = "";

// Don't call make with arguments to build in parallel
// (boolean)
const no_parallel_make: &str = "";

// Name of the rule passed to make for the install phase, default is install
// (string)
const install_rule: &str = "";

// Don't run the make install (or equivalent) stage
// (boolean)
const no_make_install: &str = "";

// Don't fix up the *.py[oc] header timestamps for ostree use.
// (boolean)
const no_python_timestamp_fix: &str = "";

// Use cmake instead of configure (deprecated: use buildsystem instead)
// (boolean)
const cmake: &str = "";

// Build system to use: autotools, cmake, cmake-ninja, meson, simple, qmake
// (string)
const buildsystem: &str = "";

// Use a build directory that is separate from the source directory
// (boolean)
const builddir: &str = "";

// Build inside this subdirectory of the extracted sources
// (string)
const subdir: &str = "";

// A build options object that can override global options
// (object)
const build_options: &str = "";

// An array of commands to run during build (between make and make install if those are used). This is primarily useful when using the "simple" buildsystem.
// Each command is run in /bin/sh -c, so it can use standard POSIX shell syntax such as piping output.
// (array of strings)
const build_commands: &str = "";

// An array of shell commands that are run after the install phase. Can for example clean up the install dir, or install extra files.
// (array of strings)
const post_install: &str = "";

// An array of file patterns that should be removed at the end. Patterns starting with / are taken to be full pathnames (without the /app prefix), otherwise
// they just match the basename. Note that any patterns will only match files installed by this module.
// (array of strings)
const cleanup: &str = "";

// The way the builder works is that files in the install directory are hard-links to the cached files, so you're not allowed to modify them in-place. If you
// list a file in this then the hardlink will be broken and you can modify it. This is a workaround, ideally installing files should replace files, not modify
// existing ones.
// (array of strings)
const ensure_writable: &str = "";

// If non-empty, only build the module on the arches listed.
// (array of strings)
const only_arches: &str = "";

// Don't build on any of the arches listed.
// (array of strings)
const skip_arches: &str = "";

// Extra files to clean up in the platform.
// (array of strings)
const cleanup_platform: &str = "";

// If true this will run the tests after installing.
// (boolean)
const run_tests: &str = "";

// The target to build when running the tests. Defaults to "check" for make and "test" for ninja. Set to empty to disable.
// (string)
const test_rule: &str = "";

// Array of commands to run during the tests.
// (array of strings)
const test_commands: &str = "";

// An array of objects specifying nested modules to be built before this one. String members in the array are interpreted as names of a separate json or yaml
// file that contains a module.
// (array of objects or strings)
const modules: &str = "";

struct Sources {
}
struct Extension {
}
struct BuildExtension {
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    return 0;
}

pub fn dump(ctx: &mut crate::execution_context::ExecutionContext) -> i32 {
    // let yml_load_result = YamlLoader::load_from_str(&ctx.content);

    // if yml_load_result.is_err() {
        // return;
    // }

    // let manifest_content = yml_load_result.unwrap();
    //
    //

    let mut lhm: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    lhm.insert(Yaml::from_str("name"), Yaml::from_str(&ctx.manifest.package_name));
    lhm.insert(Yaml::from_str("app-id"), Yaml::from_str(&ctx.manifest.package_id));
    lhm.insert(Yaml::from_str("branch"), Yaml::from_str(&ctx.manifest.package_version));
    let output_document = Yaml::Hash(lhm);

    let mut modules_to_dump: Vec<Yaml> = vec![];
    for package in &ctx.manifest.modules {
        let mut module_hash_map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
        module_hash_map.insert(Yaml::from_str("name"), Yaml::from_str(&package.name));
        module_hash_map.insert(Yaml::from_str("version"), Yaml::from_str(&package.version));
        module_hash_map.insert(Yaml::from_str("url"), Yaml::from_str(&package.url));
        let module_document = Yaml::Hash(module_hash_map);

        modules_to_dump.push(module_document);
    }

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&output_document).unwrap(); // dump the YAML object to a String
        println!("{}", out_str);
    }


    return 0;
}

pub fn is_type(ctx: &mut crate::execution_context::ExecutionContext) -> bool {
    return false;
}
