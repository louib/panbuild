use std::collections::BTreeMap;
use std::path;
use std::process::{Command, Output, Stdio};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::modules::module::{BuildSystem, SoftwareModule};

const DEFAULT_FLATPAK_BUILDER_CACHE_DIR: &str = ".flatpak-builder";
const DEFAULT_FLATPAK_OUTPUT_DIR: &str = "build";

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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub app_name: String,

    // A string defining the application id.
    // Both names (app-id and id) are accepted.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub app_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,

    // The branch to use when exporting the application.
    // If this is unset the defaults come from the default-branch option.
    //
    // This key overrides both the default-branch key, and the --default-branch commandline option.
    // Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
    // to use the default-branch key instead, because you can then override the default using
    // --default-branch when building for instance a test build.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub branch: String,

    // The default branch to use when exporting the application. Defaults to master.
    // This key can be overridden by the --default-branch commandline option.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub default_branch: String,

    // The collection ID of the repository, defaults to being unset.
    // Setting a globally unique collection ID allows the apps in the
    // repository to be shared over peer to peer systems without needing further configuration.
    // If building in an existing repository, the collection ID must match the existing
    // configured collection ID for that repository.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub collection_id: String,

    // The name of the runtime that the application uses.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub runtime: String,

    // The version of the runtime that the application uses, defaults to master.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub runtime_version: String,

    // The name of the development runtime that the application builds with.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sdk: String,

    // The name of the development extensions that the application requires to build.
    pub sdk_extensions: Vec<String>,

    // Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub var: String,

    // Use this file as the base metadata file when finishing.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub metadata: String,

    // Build a new runtime instead of an application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_runtime: Option<bool>,

    // Whether the manifest describes an extension to be used by other manifests.
    // Extensions can be used to bundle programming langages and their associated
    // tools, for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_extension: Option<bool>,

    // Start with the files from the specified application.
    // This can be used to create applications that extend another application.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub base: String,

    // Use this specific version of the application specified in base.
    // If unspecified, this uses the value specified in branch
    #[serde(skip_serializing_if = "String::is_empty")]
    pub base_version: String,

    // Install these extra extensions from the base application when
    // initializing the application directory.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub base_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or
    // sdk when finishing the build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherit_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk
    // when finishing the build, but do not inherit them into the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inherit_sdk_extensions: Vec<String>,

    // Inherit these extra extensions points from the base application or sdk when finishing the build,
    // but do not inherit them into the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_options: Option<FlatpakBuildOptions>,

    // The name of the command that the flatpak should run on execution.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub command: String,

    // Add these tags to the metadata file.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    // This is a dictionary of extension objects.
    // The key is the name of the extension.
    // See below for details.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_extensions: Vec<String>,

    // This is a dictionary of extension objects similar to add-extensions.
    // The main difference is that the extensions are added early and are
    // available for use during the build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_build_extensions: Vec<String>,

    // An array of file patterns that should be removed at the end.
    // Patterns starting with / are taken to be full pathnames (without the /app prefix),
    // otherwise they just match the basename.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup: Vec<String>,

    // An array of commandlines that are run during the cleanup phase.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_commands: Vec<String>,

    // Extra files to clean up in the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform: Vec<String>,

    // An array of commandlines that are run during the cleanup phase of the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform_commands: Vec<String>,

    // An array of commandlines that are run after importing the base platform,
    // but before applying the new files from the sdk. This is a good place to e.g. delete
    // things from the base that may conflict with the files added in the sdk.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prepare_platform_commands: Vec<String>,

    // An array of arguments passed to the flatpak build-finish command.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub finish_args: Vec<String>,

    // Any desktop file with this name will be renamed to a name
    // based on id during the cleanup phase.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_desktop_file: String,

    // Any appdata file with this name will be renamed to a name based
    // on id during the cleanup phase.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_appdata_file: String,

    // Any icon with this name will be renamed to a name based on id during
    // the cleanup phase. Note that this is the icon name, not the full filenames,
    // so it should not include a filename extension.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rename_icon: String,

    // Replace the appdata project-license field with this string.
    // This is useful as the upstream license is typically only about
    // the application itself, whereas the bundled app can contain other
    // licenses too.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub appdata_license: String,

    // If rename-icon is set, keep a copy of the old icon file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_icon: Option<bool>,

    // This string will be prefixed to the Name key in the main application desktop file.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desktop_file_name_prefix: String,

    // This string will be suffixed to the Name key in the main application desktop file.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desktop_file_name_suffix: String,

    // An array of strings specifying the modules to be built in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains a module. See below for details.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<FlatpakModule>,
}
impl FlatpakManifest {
    pub fn get_type(&self) -> &str {
        return "flatpak";
    }

    pub fn file_path_matches(path: &str) -> bool {
        let parts: Vec<&str> = path.split("/").collect();
        if parts.len() == 0 {
            return false;
        }
        let last_part = parts[parts.len() - 1].to_lowercase();
        if !last_part.ends_with("yaml") && !last_part.ends_with("yml") && !last_part.ends_with("json") {
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

    pub fn parse(manifest_content: &String) -> Option<FlatpakManifest> {
        let flatpak_manifest: FlatpakManifest = match serde_yaml::from_str(&manifest_content) {
            Ok(m) => m,
            Err(e) => {
                log::debug!("Failed to parse the Flatpak manifest: {}.", e);
                return None;
            }
        };

        // TODO I think there's other fields to validate here.
        if flatpak_manifest.app_id.is_empty() && flatpak_manifest.id.is_empty() {
            log::debug!("Required top-level field id (or app-id) is missing from flatpak manifest.");
            return None;
        }

        Some(flatpak_manifest)
    }

    pub fn dump(&self, format: &crate::manifests::manifest::ManifestFormat) -> Result<String, String> {
        if let crate::manifests::manifest::ManifestFormat::JSON = format {
            return match serde_json::to_string_pretty(&self) {
                Ok(d) => Ok(d),
                Err(e) => return Err(format!("Failed to dump the Flatpak manifest: {}.", e)),
            };
        }

        if let crate::manifests::manifest::ManifestFormat::YAML = format {
            return match serde_yaml::to_string(&self) {
                Ok(d) => Ok(d),
                Err(e) => return Err(format!("Failed to dump the Flatpak manifest: {}.", e)),
            };
        }

        Err(format!("Invalid format for Flatpak manifest."))
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    // An array of objects defining sources that will be downloaded and extracted in order.
    // String members in the array are interpreted as the name of a separate
    // json or yaml file that contains sources. See below for details.
    // FIXME this can also be a string, which represents a local path to a module file.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<FlatpakSource>,

    // An array of options that will be passed to configure
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config_opts: Vec<String>,

    // An array of arguments that will be passed to make
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_args: Vec<String>,

    // An array of arguments that will be passed to make install
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_install_args: Vec<String>,

    // If true, remove the configure script before starting build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rm_configure: Option<bool>,

    // Ignore the existence of an autogen script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_autogen: Option<bool>,

    // Don't call make with arguments to build in parallel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_parallel_make: Option<bool>,

    // Name of the rule passed to make for the install phase, default is install
    #[serde(skip_serializing_if = "String::is_empty")]
    pub install_rule: String,

    // Don't run the make install (or equivalent) stage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_make_install: Option<bool>,

    // Don't fix up the *.py[oc] header timestamps for ostree use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_python_timestamp_fix: Option<bool>,

    // Use cmake instead of configure (deprecated: use buildsystem instead)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmake: Option<bool>,

    // Build system to use: autotools, cmake, cmake-ninja, meson, simple, qmake
    #[serde(skip_serializing_if = "String::is_empty")]
    pub buildsystem: String,

    // Use a build directory that is separate from the source directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builddir: Option<bool>,

    // Build inside this subdirectory of the extracted sources
    #[serde(skip_serializing_if = "String::is_empty")]
    pub subdir: String,

    // A build options object that can override global options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_options: Option<FlatpakBuildOptions>,

    // An array of commands to run during build (between make and make install if those are used).
    // This is primarily useful when using the "simple" buildsystem.
    // Each command is run in /bin/sh -c, so it can use standard POSIX shell syntax such as piping output.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub build_commands: Vec<String>,

    // An array of shell commands that are run after the install phase.
    // Can for example clean up the install dir, or install extra files.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub post_install: Vec<String>,

    // An array of file patterns that should be removed at the end.
    // Patterns starting with / are taken to be full pathnames (without the /app prefix), otherwise
    // they just match the basename. Note that any patterns will only match
    // files installed by this module.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup: Vec<String>,

    // The way the builder works is that files in the install directory are hard-links to the cached files,
    // so you're not allowed to modify them in-place. If you list a file in this then the hardlink
    // will be broken and you can modify it. This is a workaround, ideally installing files should
    // replace files, not modify existing ones.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ensure_writable: Vec<String>,

    // If non-empty, only build the module on the arches listed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub only_arches: Vec<String>,

    // Don't build on any of the arches listed.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skip_arches: Vec<String>,

    // Extra files to clean up in the platform.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cleanup_platform: Vec<String>,

    // If true this will run the tests after installing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_tests: Option<bool>,

    // The target to build when running the tests. Defaults to "check" for make and "test" for ninja.
    // Set to empty to disable.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub test_rule: String,

    // Array of commands to run during the tests.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub test_commands: Vec<String>,

    // An array of objects specifying nested modules to be built before this one.
    // String members in the array are interpreted as names of a separate json or
    // yaml file that contains a module.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<FlatpakModule>,
}

pub const ALLOWED_SOURCE_TYPES: [&'static str; 10] = ["archive", "git", "bzr", "svn", "dir", "file", "script", "shell", "patch", "extra-data"];

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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // The name of the branch to checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<bool>,

    // If this is true, the extension is removed during when finishing.
    // This is only interesting for extensions in the add-build-extensions property.
    // Additionally the standard flatpak extension properties are supported, and put
    // directly into the metadata file: autodelete, no-autodownload, subdirectories,
    // add-ld-path, download-if, enable-if, merge-dirs, subdirectory-suffix, locale-subset,
    // version, versions. See the flatpak metadata documentation for more information on these.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cflags: String,

    // If this is true, clear cflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cflags_override: Option<bool>,

    // This is set in the environment variable CPPFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cppflags: String,

    // If this is true, clear cppflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cppflags_override: Option<bool>,

    // This is set in the environment variable CXXFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cxxflags: String,

    // If this is true, clear cxxflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxxflags_override: Option<bool>,

    // This is set in the environment variable LDFLAGS during the build.
    // Multiple specifications of this (in e.g. per-arch area) are concatenated,
    // separated by spaces.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ldflags: String,

    // If this is true, clear ldflags from previous build options before adding it from these options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldflags_override: Option<bool>,

    // The build prefix for the modules (defaults to /app for applications and /usr for runtimes).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,

    // The build libdir for the modules (defaults to /app/lib for applications and /usr/lib for runtimes).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub libdir: String,

    // This will get appended to PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_path: String,

    // This will get prepended to PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_path: String,

    // This will get appended to LD_LIBRARY_PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_ld_library_path: String,

    // This will get prepended to LD_LIBRARY_PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_ld_library_path: String,

    // This will get appended to PKG_CONFIG_PATH in the build environment (with an leading colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub append_pkg_config_path: String,

    // This will get prepended to PKG_CONFIG_PATH in the build environment (with an trailing colon if needed).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prepend_pkg_config_path: String,

    // This is a dictionary defining environment variables to be set during the build.
    // Elements in this override the properties that set the environment, like
    // cflags and ldflags. Keys with a null value unset the corresponding variable.
    pub env: BTreeMap<String, String>,

    // This is an array containing extra options to pass to flatpak build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub build_args: Vec<String>,

    // Similar to build-args but affects the tests, not the normal build.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub test_args: Vec<String>,

    // This is an array containing extra options to pass to configure.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub config_opts: Vec<String>,

    // An array of extra arguments that will be passed to make
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_args: Vec<String>,

    // An array of extra arguments that will be passed to make install
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub make_install_args: Vec<String>,

    // If this is true (the default is false) then all ELF files will be stripped after install.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<bool>,

    // By default (if strip is not true) flatpak-builder extracts all debug info in ELF files to a
    // separate files and puts this in an extension. If you want to disable this, set no-debuginfo
    // to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debuginfo: Option<bool>,

    // By default when extracting debuginfo we compress the debug sections.
    // If you want to disable this, set no-debuginfo-compression to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debuginfo_compression: Option<bool>,

    // This is a dictionary defining for each arch a separate build options object that override the main one.
    pub arch: BTreeMap<String, FlatpakBuildOptions>,
}

pub fn get_modules(manifest: &FlatpakManifest) -> Vec<SoftwareModule> {
    let mut response = vec![];
    // FIXME we should fetch those recursively.
    for module in &manifest.modules {
        let mut abstract_module = SoftwareModule::default();
        abstract_module.name = module.name.to_string();
        if module.buildsystem == "cmake" {
            abstract_module.build_system = BuildSystem::Cmake;
        }
        if module.buildsystem == "autotools" {
            abstract_module.build_system = BuildSystem::Autotools;
        }
        if module.buildsystem == "meson" {
            abstract_module.build_system = BuildSystem::Meson;
        }
        // FIXME not sure what to do with this one. Maybe we should support having a list
        // of build systems?
        if module.buildsystem == "cmake-ninja" {
            abstract_module.build_system = BuildSystem::Meson;
        }
        if module.buildsystem == "simple" {
            abstract_module.build_system = BuildSystem::Unknown;
        }
        if module.buildsystem == "qmake" {
            abstract_module.build_system = BuildSystem::Qmake;
        }

        // Skip the flatpak modules with more than 1 source, because those are harder
        // to map with a source code repository.
        if module.sources.len() != 1 {
            continue;
        }

        let sources = &module.sources[0];
        // Not handling those.
        if sources.path.is_some() {
            continue;
        }

        if sources.url.is_none() {
            continue;
        }
        abstract_module.url = sources.url.as_ref().unwrap().to_string();

        abstract_module.tag = sources.tag.as_ref().unwrap_or(&"".to_string()).to_string();

        abstract_module.config_options = module.config_opts.to_owned();
        abstract_module.build_commands = module.build_commands.to_owned();

        // TODO fetch the version from the sources.
        // FIXME should we check for duplicates here??
        response.push(abstract_module);
    }
    response
}

// Returns the updated list of modules in the manifest.
pub fn add_module(manifest: &mut FlatpakManifest, new_module: &SoftwareModule) -> Result<Vec<SoftwareModule>, String> {
    for module in &manifest.modules {
        if module.name == new_module.name {
            return Err(format!("Already a module named {}.", module.name));
        }
    }
    let mut new_flatpak_module = FlatpakModule::default();
    new_flatpak_module.name = new_module.name.to_string();

    let mut flatpak_sources = FlatpakSource::default();
    flatpak_sources.r#type = "git".to_string(); // FIXME use the url_type
    flatpak_sources.url = Some(new_module.url.to_string());
    // This is the default, unless a version is explicitely declared.
    flatpak_sources.branch = Some("master".to_string());
    new_flatpak_module.sources = vec![flatpak_sources];

    manifest.modules.insert(0, new_flatpak_module);

    Ok(get_modules(manifest))
}

pub fn run_build(abstract_manifest: &crate::manifests::manifest::AbstractManifest) -> Result<String, String> {
    let flatpak_cache_dir = path::Path::new(DEFAULT_FLATPAK_BUILDER_CACHE_DIR);
    if flatpak_cache_dir.is_dir() {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        let backup_folder_name = format!("{}-{}", DEFAULT_FLATPAK_BUILDER_CACHE_DIR.to_owned(), timestamp.unwrap().as_secs());
        println!("Making a backup of the flatpak-builder cache folder at {}", backup_folder_name);

        let mut output = Command::new("cp")
            .arg("-R")
            .arg(DEFAULT_FLATPAK_BUILDER_CACHE_DIR)
            .arg(backup_folder_name)
            .spawn();

        let mut output = match output {
            Ok(o) => o,
            Err(e) => return Err(e.to_string()),
        };
    }

    let child = Command::new("flatpak-builder")
        .arg("--user")
        .arg("--force-clean")
        // .arg("-v")
        .arg("--keep-build-dirs")
        .arg(DEFAULT_FLATPAK_OUTPUT_DIR)
        .arg(&abstract_manifest.path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    if !output.status.success() {
        return Ok("it went ok".to_string());
    }
    Ok(String::from("lol"))
}

pub fn run_command(abstract_manifest: &crate::manifests::manifest::AbstractManifest, command: &str) -> Result<String, String> {
    let flatpak_build_dir = path::Path::new(DEFAULT_FLATPAK_OUTPUT_DIR);
    if !flatpak_build_dir.is_dir() {
        return Err("Looks like this workspace was not built. Run `panbuild make` first.".to_string());
    }

    let child = Command::new("flatpak-builder")
        .arg("--run")
        .arg(DEFAULT_FLATPAK_OUTPUT_DIR)
        .arg(&abstract_manifest.path)
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    if !output.status.success() {
        return Ok("it went ok".to_string());
    }
    Ok(String::from("lol"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(FlatpakManifest::file_path_matches("com.example.appName.yaml"));
        assert!(FlatpakManifest::file_path_matches("io.github.user.repo.Devel.yaml"));
        assert!(FlatpakManifest::file_path_matches("/path/to/com.example.appName.yaml"));
        assert!(FlatpakManifest::file_path_matches("/path/to/com.example.appName.yml"));
        assert!(FlatpakManifest::file_path_matches("/path/to/com.example.department.product.yaml"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/file.yaml"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/file.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/___432423fdsf.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/example.com.json"));
        assert!(!FlatpakManifest::file_path_matches("/path/to/example.com.json."));
        assert!(!FlatpakManifest::file_path_matches(""));
        assert!(!FlatpakManifest::file_path_matches("/////////////"));
    }

    #[test]
    #[should_panic]
    pub fn test_parse_invalid_yaml() {
        FlatpakManifest::parse(&"----------------------------".to_string()).unwrap();
    }

    #[test]
    pub fn test_parse_missing_fields() {
        assert!(FlatpakManifest::parse(
            &r###"
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: panbuild
        "###
            .to_string(),
        )
        .is_none());
    }

    #[test]
    pub fn test_parse() {
        match FlatpakManifest::parse(
            &r###"
            app-id: net.louib.panbuild
            runtime: org.gnome.Platform
            runtime-version: "3.36"
            sdk: org.gnome.Sdk
            command: panbuild
            tags: ["nightly"]
            modules:
              -
                name: "panbuild"
                buildsystem: simple
                cleanup: [ "*" ]
                config-opts: []
                sources:
                  -
                    type: git
                    url: https://github.com/louib/panbuild.git
                    branch: master
        "###
            .to_string(),
        ) {
            None => panic!("Error while parsing the flatpak manifest."),
            Some(manifest) => {
                assert_eq!(manifest.app_id, "net.louib.panbuild");
            }
        }
    }
}
