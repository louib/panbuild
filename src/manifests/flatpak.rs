use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// Other choices are org.gnome.Platform and org.kde.Platform
const DEFAULT_RUNTIME: &str = "org.freedesktop.Platform";
const DEFAULT_RUNTIME_VERSION: &str = "master";
// Other choices are org.gnome.Sdk and org.kde.Sdk
const DEFAULT_SDK: &str = "org.freedesktop.Sdk";

// See `man flatpak-manifest` for the flatpak manifest specs.
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct FlatpakManifest {
    // Name of the application.
    pub app_name: String,

    // A string defining the application id.
    // Both names (app-id and id) are accepted.
    pub app_id: String,
    pub id: String,

    // The branch to use when exporting the application.
    // If this is unset the defaults come from the default-branch option.
    //
    // This key overrides both the default-branch key, and the --default-branch commandline option.
    // Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
    // to use the default-branch key instead, because you can then override the default using
    // --default-branch when building for instance a test build.
    pub branch: String,

    // The default branch to use when exporting the application. Defaults to master.
    // This key can be overridden by the --default-branch commandline option.
    pub default_branch: String,

    // The collection ID of the repository, defaults to being unset.
    // Setting a globally unique collection ID allows the apps in the
    // repository to be shared over peer to peer systems without needing further configuration.
    // If building in an existing repository, the collection ID must match the existing
    // configured collection ID for that repository.
    pub collection_id: String,

    // The name of the runtime that the application uses.
    pub runtime: String,

    // The version of the runtime that the application uses, defaults to master.
    pub runtime_version: String,

    // The name of the development runtime that the application builds with.
    pub sdk: String,

    // Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
    pub var: String,

    // Use this file as the base metadata file when finishing.
    pub metadata: String,

    // Build a new runtime instead of an application.
    pub build_runtime: Option<bool>,

    // Whether the manifest describes an extension to be used by other manifests.
    // Extensions can be used to bundle programming langages and their associated
    // tools, for example.
    pub build_extension: Option<bool>,

    // Start with the files from the specified application.
    // This can be used to create applications that extend another application.
    pub base: String,

    // Use this specific version of the application specified in base.
    // If unspecified, this uses the value specified in branch
    pub base_version: String,

    // Install these extra extensions from the base application when
    // initializing the application directory.
    pub base_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or
    // sdk when finishing the build.
    pub inherit_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk
    // when finishing the build, but do not inherit them into the platform.
    pub inherit_sdk_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    pub build_options: Option<FlatpakBuildOptions>,

    // Add these tags to the metadata file.
    pub tags: Vec<String>,

    // An array of strings specifying the modules to be built in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains a module. See below for details.
    pub modules: Vec<FlatpakModule>,

    // This is a dictionary of extension objects.
    // The key is the name of the extension.
    // See below for details.
    pub add_extensions: Vec<String>,

    // This is a dictionary of extension objects similar to add-extensions.
    // The main difference is that the extensions are added early and are
    // available for use during the build.
    pub add_build_extensions: Vec<String>,

    // An array of file patterns that should be removed at the end.
    // Patterns starting with / are taken to be full pathnames (without the /app prefix),
    // otherwise they just match the basename.
    pub cleanup: Vec<String>,

    // An array of commandlines that are run during the cleanup phase.
    pub cleanup_commands: Vec<String>,

    // Extra files to clean up in the platform.
    pub cleanup_platform: Vec<String>,

    // An array of commandlines that are run during the cleanup phase of the platform.
    pub cleanup_platform_commands: Vec<String>,

    // An array of commandlines that are run after importing the base platform,
    // but before applying the new files from the sdk. This is a good place to e.g. delete
    // things from the base that may conflict with the files added in the sdk.
    pub prepare_platform_commands: Vec<String>,

    // An array of arguments passed to the flatpak build-finish command.
    pub finish_args: Vec<String>,

    // Any desktop file with this name will be renamed to a name
    // based on id during the cleanup phase.
    pub rename_desktop_file: String,

    // Any appdata file with this name will be renamed to a name based
    // on id during the cleanup phase.
    pub rename_appdata_file: String,

    // Any icon with this name will be renamed to a name based on id during
    // the cleanup phase. Note that this is the icon name, not the full filenames,
    // so it should not include a filename extension.
    pub rename_icon: String,

    // Replace the appdata project-license field with this string.
    // This is useful as the upstream license is typically only about
    // the application itself, whereas the bundled app can contain other
    // licenses too.
    pub appdata_license: String,

    // If rename-icon is set, keep a copy of the old icon file.
    pub copy_icon: Option<bool>,

    // This string will be prefixed to the Name key in the main application desktop file.
    pub desktop_file_name_prefix: String,

    // This string will be suffixed to the Name key in the main application desktop file.
    pub desktop_file_name_suffix: String,
}

// Each module specifies a source that has to be separately built and installed.
// It contains the build options and a list of sources to download and extract before
// building.
//
// Modules can be nested, in order to turn related modules on and off with a single key.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct FlatpakModule {
    // The name of the module, used in e.g. build logs. The name is also
    // used for constructing filenames and commandline arguments,
    // therefore using spaces or '/' in this string is a bad idea.
    pub name: String,

    // If true, skip this module
    pub disabled: Option<bool>,

    // An array of objects defining sources that will be downloaded and extracted in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains sources. See below for details.
    // FIXME this can also be a string, which represents a local path to a module file.
    pub sources: Vec<FlatpakSource>,

    // An array of options that will be passed to configure
    pub config_opts: Vec<String>,

    // An array of arguments that will be passed to make
    pub make_args: Vec<String>,

    // An array of arguments that will be passed to make install
    pub make_install_args: Vec<String>,

    // If true, remove the configure script before starting build
    pub rm_configure: Option<bool>,

    // Ignore the existence of an autogen script
    pub no_autogen: Option<bool>,

    // Don't call make with arguments to build in parallel
    pub no_parallel_make: Option<bool>,

    // Name of the rule passed to make for the install phase, default is install
    pub install_rule: String,

    // Don't run the make install (or equivalent) stage
    pub no_make_install: Option<bool>,

    // Don't fix up the *.py[oc] header timestamps for ostree use.
    pub no_python_timestamp_fix: Option<bool>,

    // Use cmake instead of configure (deprecated: use buildsystem instead)
    pub cmake: Option<bool>,

    // Build system to use: autotools, cmake, cmake-ninja, meson, simple, qmake
    pub buildsystem: String,

    // Use a build directory that is separate from the source directory
    pub builddir: Option<bool>,

    // Build inside this subdirectory of the extracted sources
    pub subdir: String,

    // A build options object that can override global options
    pub build_options: Option<FlatpakBuildOptions>,

    // An array of commands to run during build (between make and make install if those are used).
    // This is primarily useful when using the "simple" buildsystem.
    // Each command is run in /bin/sh -c, so it can use standard POSIX shell syntax such as piping output.
    pub build_commands: Vec<String>,

    // An array of shell commands that are run after the install phase.
    // Can for example clean up the install dir, or install extra files.
    pub post_install: Vec<String>,

    // An array of file patterns that should be removed at the end.
    // Patterns starting with / are taken to be full pathnames (without the /app prefix), otherwise
    // they just match the basename. Note that any patterns will only match
    // files installed by this module.
    pub cleanup: Vec<String>,

    // The way the builder works is that files in the install directory are hard-links to the cached files,
    // so you're not allowed to modify them in-place. If you list a file in this then the hardlink
    // will be broken and you can modify it. This is a workaround, ideally installing files should
    // replace files, not modify existing ones.
    pub ensure_writable: Vec<String>,

    // If non-empty, only build the module on the arches listed.
    pub only_arches: Vec<String>,

    // Don't build on any of the arches listed.
    pub skip_arches: Vec<String>,

    // Extra files to clean up in the platform.
    pub cleanup_platform: Vec<String>,

    // If true this will run the tests after installing.
    pub run_tests: Option<bool>,

    // The target to build when running the tests. Defaults to "check" for make and "test" for ninja.
    // Set to empty to disable.
    pub test_rule: String,

    // Array of commands to run during the tests.
    pub test_commands: Vec<String>,

    // An array of objects specifying nested modules to be built before this one.
    // String members in the array are interpreted as names of a separate json or
    // yaml file that contains a module.
    pub modules: Vec<FlatpakModule>,
}

// The sources are a list pointer to the source code that needs to be extracted into
// the build directory before the build starts.
// They can be of several types, distinguished by the type property.
//
// Additionally, the sources list can contain a plain string, which is interpreted as the name
// of a separate json or yaml file that is read and inserted at this
// point. The file can contain a single source, or an array of sources.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FlatpakSource {
    // Allowed source types are:
    //   * archive,
    //   * git,
    //   * bzr,
    //   * svn,
    //   * dir,
    //   * file,
    //   * script,
    //   * shell,
    //   * patch,
    //   * extra-data,
    pub r#type: String,
    pub url: String,
    pub tag: Option<String>,
}

// Extension define extension points in the app/runtime that can be implemented by extensions,
// supplying extra files which are available during runtime..
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct FlatpakExtension {
    // The directory where the extension is mounted. If the extension point is for an application,
    // this path is relative to /app, otherwise it is relative to /usr.
    pub extension_directory: String,

    // If this is true, then the data created in the extension directory is omitted from the result,
    // and instead packaged in a separate extension.
    pub bundle: Option<bool>,

    // If this is true, the extension is removed during when finishing.
    // This is only interesting for extensions in the add-build-extensions property.
    // Additionally the standard flatpak extension properties are supported, and put
    // directly into the metadata file: autodelete, no-autodownload, subdirectories,
    // add-ld-path, download-if, enable-if, merge-dirs, subdirectory-suffix, locale-subset,
    // version, versions. See the flatpak metadata documentation for more information on these.
    pub remove_after_build: Option<bool>,
}

// Build options specify the build environment of a module,
// and can be specified globally as well as per-module.
// Options can also be specified on a per-architecture basis using the arch property.
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct FlatpakBuildOptions {
    // This is set in the environment variable CFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    pub cflags: String,

    // If this is true, clear cflags from previous build options before adding it from these options.
    pub cflags_override: Option<bool>,

    // This is set in the environment variable CPPFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    pub cppflags: String,

    // If this is true, clear cppflags from previous build options before adding it from these options.
    pub cppflags_override: Option<bool>,

    // This is set in the environment variable CXXFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    pub cxxflags: String,

    // If this is true, clear cxxflags from previous build options before adding it from these options.
    pub cxxflags_override: Option<bool>,

    // This is set in the environment variable LDFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated,
    // separated by spaces.
    pub ldflags: String,

    // If this is true, clear ldflags from previous build options before adding it from these options.
    pub ldflags_override: Option<bool>,

    // The build prefix for the modules (defaults to /app for applications and /usr for runtimes).
    pub prefix: String,

    // The build libdir for the modules (defaults to /app/lib for applications and /usr/lib for runtimes).
    pub libdir: String,

    // This will get appended to PATH in the build environment (with an leading colon if needed).
    pub append_path: String,

    // This will get prepended to PATH in the build environment (with an trailing colon if needed).
    pub prepend_path: String,

    // This will get appended to LD_LIBRARY_PATH in the build environment (with an leading colon if needed).
    pub append_ld_library_path: String,

    // This will get prepended to LD_LIBRARY_PATH in the build environment (with an trailing colon if needed).
    pub prepend_ld_library_path: String,

    // This will get appended to PKG_CONFIG_PATH in the build environment (with an leading colon if needed).
    pub append_pkg_config_path: String,

    // This will get prepended to PKG_CONFIG_PATH in the build environment (with an trailing colon if needed).
    pub prepend_pkg_config_path: String,

    // This is a dictionary defining environment variables to be set during the build.
    // Elements in this override the properties that set the environment, like
    // cflags and ldflags. Keys with a null value unset the corresponding variable.
    pub build_env: BTreeMap<String, String>,

    // This is an array containing extra options to pass to flatpak build.
    pub build_args: Vec<String>,

    // Similar to build-args but affects the tests, not the normal build.
    pub test_args: Vec<String>,

    // This is an array containing extra options to pass to configure.
    pub config_opts: Vec<String>,

    // An array of extra arguments that will be passed to make
    pub make_args: Vec<String>,

    // An array of extra arguments that will be passed to make install
    pub make_install_args: Vec<String>,

    // If this is true (the default is false) then all ELF files will be stripped after install.
    pub strip: Option<bool>,

    // By default (if strip is not true) flatpak-builder extracts all debug info in ELF files to a
    // separate files and puts this in an extension. If you want to disable this, set no-debuginfo
    // to true.
    pub no_debuginfo: Option<bool>,

    // By default when extracting debuginfo we compress the debug sections.
    // If you want to disable this, set no-debuginfo-compression to true.
    pub no_debuginfo_compression: Option<bool>,

    // This is a dictionary defining for each arch a separate build options object that override the main one.
    pub arch: BTreeMap<String, FlatpakBuildOptions>,
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) {
    ctx.manifest = crate::manifests::manifest::AbstractManifest::default();
    // TODO actually handle the error.
    let flatpak_manifest: FlatpakManifest = match serde_yaml::from_str(&ctx.content) {
        Ok(m) => m,
        Err(e) => panic!("Failed to parse the Flatpak manifest: {}.", e),
    };

    ctx.manifest.flatpak_manifest = Some(flatpak_manifest);
}

pub fn dump_native(abstract_manifest: &crate::manifests::manifest::AbstractManifest) -> String {
    let flatpak_manifest = match &abstract_manifest.flatpak_manifest {
        Some(m) => m,
        None => panic!("Called flatpak.dump_native without a native Flatpak manifest!"),
    };


    if let crate::manifests::manifest::ManifestFormat::JSON = abstract_manifest.format {
        let manifest_dump: String = match serde_json::to_string(&flatpak_manifest) {
            Ok(d) => d,
            Err(e) => panic!("Failed to dump the Flatpak manifest: {}.", e),
        };
        return manifest_dump;
    }

    // TODO we might want to assert that the format is YAML before dumping.
    // Assuming default YAML format.
    let manifest_dump: String = match serde_yaml::to_string(&flatpak_manifest) {
        Ok(d) => d,
        Err(e) => panic!("Failed to dump the Flatpak manifest: {}.", e),
    };
    manifest_dump
}

pub fn file_path_matches(path: &str) -> bool {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() == 0 {
        return false;
    }
    let last_part = parts[parts.len() - 1];
    if !last_part.to_lowercase().ends_with("yaml") && !last_part.to_lowercase().ends_with("json") {
        return false;
    }
    let mut dot_count = 0;
    for c in last_part.chars() {
        if c == '.' {
            dot_count = dot_count + 1;
            continue;
        }
        if c.is_alphabetic() || c.is_numeric() {
            continue;
        }
        return false;
    }
    // The reverse DNS notation is used for the
    // flatpak app IDs and the associated manifest
    // files. This means at least 3 dots in the
    // resulting name.
    if dot_count < 3 {
        return false;
    }
    return true;
}

pub fn file_content_matches(content: &str) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(file_path_matches("com.example.appName.yaml"));
        assert!(file_path_matches("/path/to/com.example.appName.yaml"));
        assert!(file_path_matches("/path/to/com.example.department.product.yaml"));
        assert!(!file_path_matches("/path/to/file.yaml"));
        assert!(!file_path_matches("/path/to/file.json"));
        assert!(!file_path_matches("/path/to/___432423fdsf.json"));
        assert!(!file_path_matches("/path/to/example.com.json"));
        assert!(!file_path_matches("/path/to/example.com.json."));
        assert!(!file_path_matches(""));
        assert!(!file_path_matches("/////////////"));
    }
}
