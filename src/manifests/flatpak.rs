use std::collections::BTreeMap;

extern crate yaml_rust;
extern crate linked_hash_map;

use linked_hash_map::{LinkedHashMap};
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};


// Other choices are org.gnome.Platform and org.kde.Platform
const DEFAULT_RUNTIME: &str = "org.freedesktop.Platform";
const DEFAULT_RUNTIME_VERSION: &str = "master";
// Other choices are org.gnome.Sdk and org.kde.Sdk
const DEFAULT_SDK: &str = "org.freedesktop.Sdk";

// See `man flatpak-manifest` for the flatpak manifest specs.

// **** Top-level Fields
// Name of the application.
// string
const APP_NAME: &str = "app-name";

// A string defining the application id.
// string
const APP_ID: &str = "app-id";

// The branch to use when exporting the application.
// If this is unset the defaults come from the default-branch option.
//
// This key overrides both the default-branch key, and the --default-branch commandline option.
// Unless you need a very specific branchname (like for a runtime or an extension) it is recommended
// to use the default-branch key instead, because you can then override the default using
// --default-branch when building for instance a test build.
// string
const BRANCH: &str = "branch";

// The default branch to use when exporting the application. Defaults to master.
// This key can be overridden by the --default-branch commandline option.
// string
const DEFAULT_BRANCH: &str = "default-branch";

// The collection ID of the repository, defaults to being unset.
// Setting a globally unique collection ID allows the apps in the
// repository to be shared over peer to peer systems without needing further configuration.
// If building in an existing repository, the collection ID must match the existing
// configured collection ID for that repository.
// string
const COLLECTION_ID: &str = "collection-id";

// The name of the runtime that the application uses.
// string
const RUNTIME: &str = "runtime";

// The version of the runtime that the application uses, defaults to master.
// string
const RUNTIME_VERSION: &str = "runtime-version";

// The name of the development runtime that the application builds with.
// string
const SDK: &str = "sdk";

// Initialize the (otherwise empty) writable /var in the build with a copy of this runtime.
// string
const VAR: &str = "var";

// Use this file as the base metadata file when finishing.
// string
const METADATA: &str = "metadata";

// Build a new runtime instead of an application.
// bool
const BUILD_RUNTIME: &str = "build-runtime";

// Build an extension.
// bool
const BUILD_EXTENSION: &str = "build-extension";

// Start with the files from the specified application.
// This can be used to create applications that extend another application.
// string
const BASE: &str = "base";

// Use this specific version of the application specified in base.
// If unspecified, this uses the value specified in branch
// string
const BASE_VERSION: &str = "base-version";

// Install these extra extensions from the base application when
// initializing the application directory.
// list of strings
const BASE_EXTENSIONS: &str = "base-extensions";

// Inherit these extra extensions points from the base application or
// sdk when finishing the build.
// list of strings
const INHERIT_EXTENSIONS: &str = "inherit-extensions";

// Inherit these extra extensions points from the base application or sdk
// when finishing the build, but do not inherit them into the platform.
// list of strings
const INHERIT_SDK_EXTENSIONS: &str = "inherit-sdk-extensions";

// Inherit these extra extensions points from the base application or sdk when finishing the build,
// but do not inherit them into the platform.
// list of strings
// const BUILD_OPTIONS: &str = "build-options";

// Add these tags to the metadata file.
// list of strings
const TAGS: &str = "tags";

// An array of strings specifying the modules to be built in order.
// String members in the array are interpreted as the name of a separate
// json or yaml file that contains a module. See below for details.
// list of strings
const MODULES: &str = "modules";

// This is a dictionary of extension objects.
// The key is the name of the extension.
// See below for details.
// list of strings
const ADD_EXTENSIONS: &str = "add-extensions";

// This is a dictionary of extension objects similar to add-extensions.
// The main difference is that the extensions are added early and are
// available for use during the build.
// list of strings
const ADD_BUILD_EXTENSIONS: &str = "add-build-extensions";

// An array of file patterns that should be removed at the end.
// Patterns starting with / are taken to be full pathnames (without the /app prefix),
// otherwise they just match the basename.
// list of strings
const CLEANUP: &str = "cleanup";

// An array of commandlines that are run during the cleanup phase.
// list of strings
const CLEANUP_COMMANDS: &str = "cleanup-commands";

// Extra files to clean up in the platform.
// list of strings
const CLEANUP_PLATFORM: &str = "cleanup-platform";

// An array of commandlines that are run during the cleanup phase of the platform.
// list of strings
const CLEANUP_PLATFORM_COMMANDS: &str = "cleanup-platform-commands";

// An array of commandlines that are run after importing the base platform,
// but before applying the new files from the sdk. This is a good place to e.g. delete
// things from the base that may conflict with the files added in the sdk.
// list of strings
const PREPARE_PLATFORM_COMMANDS: &str = "prepare-platform-commands";

// An array of arguments passed to the flatpak build-finish command.
// list of strings
const FINISH_ARGS: &str = "finish-args";

// Any desktop file with this name will be renamed to a name
// based on id during the cleanup phase.
// string
const RENAME_DESKTOP_FILE: &str = "rename-desktop-file";

// Any appdata file with this name will be renamed to a name based
// on id during the cleanup phase.
// string
const RENAME_APPDATA_FILE: &str = "rename-appdata-file";

// Any icon with this name will be renamed to a name based on id during
// the cleanup phase. Note that this is the icon name, not the full filenames,
// so it should not include a filename extension.
// string
const RENAME_ICON: &str = "rename-icon";

// Replace the appdata project-license field with this string.
// This is useful as the upstream license is typically only about
// the application itself, whereas the bundled app can contain other
// licenses too.
// string
const APPDATA_LICENSE: &str = "appdata-license";

// If rename-icon is set, keep a copy of the old icon file.
// bool
const COPY_ICON: &str = "copy-icon";

// This string will be prefixed to the Name key in the main application desktop file.
// string
const DESKTOP_FILE_NAME_PREFIX: &str = "desktop-file-name-prefix";

// This string will be suffixed to the Name key in the main application desktop file.
// string
const DESKTOP_FILE_NAME_SUFFIX: &str = "desktop-file-name-suffix";



// **** Module Fields.
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
const MODULE_NAME: &str = "name";

// If true, skip this module
// (boolean)
const MODULE_DISABLED: &str = "disabled";

// An array of objects defining sources that will be downloaded and extracted in order. String members in the array are interpreted as the name of a separate
// json or yaml file that contains sources. See below for details.
// (array of objects or strings)
const SOURCES: &str = "sources";

// An array of options that will be passed to configure
// (array of strings)
// const CONFIG_OPTS: &str = "";

// An array of arguments that will be passed to make
// (array of strings)
// const MAKE_ARGS: &str = "";

// An array of arguments that will be passed to make install
// (array of strings)
// const MAKE_INSTALL_ARGS: &str = "";
// FIXME why is this also defined in the build options?

// If true, remove the configure script before starting build
// (boolean)
const RM_CONFIGURE: &str = "rm-configure";

// Ignore the existence of an autogen script
// (boolean)
const NO_AUTOGEN: &str = "no-autogen";

// Don't call make with arguments to build in parallel
// (boolean)
const NO_PARALLEL_MAKE: &str = "no-parallel-make";

// Name of the rule passed to make for the install phase, default is install
// (string)
const INSTALL_RULE: &str = "install-rule";

// Don't run the make install (or equivalent) stage
// (boolean)
const NO_MAKE_INSTALL: &str = "no-make-install";

// Don't fix up the *.py[oc] header timestamps for ostree use.
// (boolean)
const NO_PYTHON_TIMESTAMP_FIX: &str = "no-python-timestamp-fix";

// Use cmake instead of configure (deprecated: use buildsystem instead)
// (boolean)
const CMAKE: &str = "cmake";

// Build system to use: autotools, cmake, cmake-ninja, meson, simple, qmake
// (string)
const BUILDSYSTEM: &str = "buildsystem";

// Use a build directory that is separate from the source directory
// (boolean)
const BUILDDIR: &str = "builddir";

// Build inside this subdirectory of the extracted sources
// (string)
const SUBDIR: &str = "subdir";

// A build options object that can override global options
// (object)
const BUILD_OPTIONS: &str = "build-options";

// An array of commands to run during build (between make and make install if those are used).
// This is primarily useful when using the "simple" buildsystem.
// Each command is run in /bin/sh -c, so it can use standard POSIX shell syntax such as piping output.
// (array of strings)
const BUILD_COMMANDS: &str = "build-commands";

// An array of shell commands that are run after the install phase. Can for example clean up the install dir, or install extra files.
// (array of strings)
const POST_INSTALL: &str = "post-install";

// An array of file patterns that should be removed at the end. Patterns starting with / are taken to be full pathnames (without the /app prefix), otherwise
// they just match the basename. Note that any patterns will only match files installed by this module.
// (array of strings)
// const CLEANUP: &str = "cleanup";

// The way the builder works is that files in the install directory are hard-links to the cached files, so you're not allowed to modify them in-place. If you
// list a file in this then the hardlink will be broken and you can modify it. This is a workaround, ideally installing files should replace files, not modify
// existing ones.
// (array of strings)
const ENSURE_WRITABLE: &str = "ensure-writable";

// If non-empty, only build the module on the arches listed.
// (array of strings)
const ONLY_ARCHES: &str = "only-arches";

// Don't build on any of the arches listed.
// (array of strings)
const SKIP_ARCHES: &str = "skip-arches";

// Extra files to clean up in the platform.
// (array of strings)
// const CLEANUP_PLATFORM: &str = "cleanup-platform";

// If true this will run the tests after installing.
// (boolean)
const RUN_TESTS: &str = "run-tests";

// The target to build when running the tests. Defaults to "check" for make and "test" for ninja. Set to empty to disable.
// (string)
const TEST_RULE: &str = "test-rule";

// Array of commands to run during the tests.
// (array of strings)
const TEST_COMMANDS: &str = "test-commands";

// An array of objects specifying nested modules to be built before this one.
// String members in the array are interpreted as names of a separate json or yaml file that contains a module.
// (array of objects or strings)
// TODO extract this
// const MODULES: &str = "modules";




// **** Sources
// The sources are a list pointer to the source code that  needs to be extracted into the build directory before the build starts.
// They can be of several types, distinguished by the type property.
//
// Additionally, the sources list can contain a plain string, which is interpreted as the name
// of a separate json or yaml file that is read and inserted at this
// point. The file can contain a single source, or an array of sources.
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
const SOURCE_TYPE: &str = "type";


// **** Extensions
// Extension define extension points in the app/runtime that can be implemented by extensions,
// supplying extra files which are available during runtime..
//
// The directory where the extension is mounted. If the extension point is for an application,
// this path is relative to /app, otherwise it is relative to /usr.
// (string)
const EXTENSION_DIRECTORY: &str = "directory";

// If this is true, then the data created in the extension directory is omitted from the result, and instead packaged in a separate extension.
// (boolean)
const BUNDLE: &str = "bundle";

// If this is true, the extension is removed during when finishing. This is only interesting for extensions in the add-build-extensions property.

// Additionally the standard flatpak extension properties are supported, and put directly into the metadata file: autodelete, no-autodownload, subdirectories,
// add-ld-path, download-if, enable-if, merge-dirs, subdirectory-suffix, locale-subset, version, versions. See the flatpak metadata documentation for more
// information on these.
// (boolean)
const REMOVE_AFTER_BUILD: &str = "remove-after-build";




// **** Build Options
// Build options specify the build environment of a module, and can be specified globally as well as per-module.
// Options can also be specified on a per-architecture basis using the arch property.

// This is set in the environment variable CFLAGS during the build. Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by
// spaces.
// (string)
const CFLAGS: &str = "cflags";


// If this is true, clear cflags from previous build options before adding it from these options.
// (boolean)
const CFLAGS_OVERRIDE: &str = "cflags-override";


// This is set in the environment variable CPPFLAGS during the build. Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by
// spaces.
// (string)
const CPPFLAGS: &str = "cppflags";


// If this is true, clear cppflags from previous build options before adding it from these options.
// (boolean)
const CPPFLAGS_OVERRIDE: &str = "cppflags-override";


// This is set in the environment variable CXXFLAGS during the build. Multiple specifications of this (in e.g. per-arch area) are concatenated, separated by
// spaces.
// (string)
const CXXFLAGS: &str = "cxxflags";


// If this is true, clear cxxflags from previous build options before adding it from these options.
// (boolean)
const CXXFLAGS_OVERRIDE: &str = "cxxflags-override";


// This is set in the environment variable LDFLAGS during the build.
// Multiple specifications of this (in e.g. per-arch area) are concatenated,
// separated by spaces.
// (string)
const LDFLAGS: &str = "ldflags";


// If this is true, clear ldflags from previous build options before adding it from these options.
// (boolean)
const LDFLAGS_OVERRIDE: &str = "ldflags-override";


// The build prefix for the modules (defaults to /app for applications and /usr for runtimes).
// (string)
const PREFIX: &str = "prefix";


// The build libdir for the modules (defaults to /app/lib for applications and /usr/lib for runtimes).
// (string)
const LIBDIR: &str = "libdir";


// This will get appended to PATH in the build environment (with an leading colon if needed).
// (string)
const APPEND_PATH: &str = "append-path";


// This will get prepended to PATH in the build environment (with an trailing colon if needed).
// (string)
const PREPEND_PATH: &str = "prepend-path";


// This will get appended to LD_LIBRARY_PATH in the build environment (with an leading colon if needed).
// (string)
const APPEND_LD_LIBRARY_PATH: &str = "append-ld-library-path";


// This will get prepended to LD_LIBRARY_PATH in the build environment (with an trailing colon if needed).
// (string)
const PREPEND_LD_LIBRARY_PATH: &str = "prepend-ld-library-path";


// This will get appended to PKG_CONFIG_PATH in the build environment (with an leading colon if needed).
// (string)
const APPEND_PKG_CONFIG_PATH: &str = "append-pkg-config-path";


// This will get prepended to PKG_CONFIG_PATH in the build environment (with an trailing colon if needed).
// (string)
const PREPEND_PKG_CONFIG_PATH: &str = "prepend-pkg-config-path";

// This is a dictionary defining environment variables to be set during the build.
// Elements in this override the properties that set the environment, like
// cflags and ldflags. Keys with a null value unset the corresponding variable.
// (object)
const BUILD_ENV: &str = "env";

// This is an array containing extra options to pass to flatpak build.
// (array of strings)
const BUILD_ARGS: &str = "build-args";

// Similar to build-args but affects the tests, not the normal build.
// (array of strings)
const TEST_ARGS: &str = "test-args";

// This is an array containing extra options to pass to configure.
// (array of strings)
const CONFIG_OPTS: &str = "config-opts";

// An array of extra arguments that will be passed to make
// (array of strings)
const MAKE_ARGS: &str = "make-args";

// An array of extra arguments that will be passed to make install
// (array of strings)
const MAKE_INSTALL_ARGS: &str = "make-install-args";

// If this is true (the default is false) then all ELF files will be stripped after install.
// (boolean)
const STRIP: &str = "strip";

// By default (if strip is not true) flatpak-builder extracts all debug info in ELF files to a
// separate files and puts this in an extension. If you want to disable this, set no-debuginfo
// to true.
// (boolean)
const NO_DEBUGINFO: &str = "no-debuginfo";

// By default when extracting debuginfo we compress the debug sections.
// If you want to disable this, set no-debuginfo-compression to true.
// (boolean)
const NO_DEBUGINFO_COMPRESSION: &str = "no-debuginfo-compression";

// This is a dictionary defining for each arch a separate build options object that override the main one.
// (object)
const ARCH: &str = "arch";


pub fn parse(content: &str) -> crate::manifests::manifest::AbstractManifest {
    let mut response = crate::manifests::manifest::AbstractManifest::default();

    let yml_load_result = YamlLoader::load_from_str(&content);
    if yml_load_result.is_err() {
        eprintln!("Could not parse yaml file");
        return response;
    }

    let manifest_content = &yml_load_result.unwrap()[0];

    return response;
}

pub fn dump_module(module: &crate::manifests::manifest::AbstractModule) -> Yaml {
    let mut module_hash_map: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    module_hash_map.insert(Yaml::from_str(APP_NAME), Yaml::from_str(&module.name));
    module_hash_map.insert(Yaml::from_str(RUNTIME_VERSION), Yaml::from_str(&module.version));
    let module_document = Yaml::Hash(module_hash_map);

    return module_document;
    // return Yaml::from_str("");
}

pub fn dump(manifest: &crate::manifests::manifest::AbstractManifest) -> String {
    let mut lhm: LinkedHashMap<Yaml, Yaml> = LinkedHashMap::new();
    lhm.insert(Yaml::from_str(APP_NAME), Yaml::from_str(&manifest.package_name));
    lhm.insert(Yaml::from_str(APP_ID), Yaml::from_str(&manifest.package_id));
    lhm.insert(Yaml::from_str(DEFAULT_BRANCH), Yaml::from_str(&manifest.package_version));

    lhm.insert(Yaml::from_str(RUNTIME), Yaml::from_str(DEFAULT_RUNTIME));
    lhm.insert(Yaml::from_str(RUNTIME_VERSION), Yaml::from_str(DEFAULT_RUNTIME_VERSION));
    lhm.insert(Yaml::from_str(SDK), Yaml::from_str(DEFAULT_SDK));

    // I don't think we're going to use flatpak to build extensions.
    lhm.insert(Yaml::from_str(BUILD_EXTENSION), Yaml::Boolean(false));

    let mut tags = [].to_vec();
    for keyword in &manifest.keywords {
        tags.push(Yaml::from_str(&keyword));
    }

    // TODO add language specific extensions, like rust, with the BASE_EXTENSIONS field.

    let output_document = Yaml::Hash(lhm);

    let mut modules_to_dump: Vec<Yaml> = vec![];
    for module in &manifest.modules {
        modules_to_dump.push(dump_module(module));
    }

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&output_document).unwrap(); // dump the YAML object to a String
    }


    return out_str;
}

pub fn file_path_matches(path: &str) -> bool {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() == 0 {
        return false
    }
    let last_part = parts[parts.len() - 1];
    if ! last_part.to_lowercase().ends_with("yaml") && ! last_part.to_lowercase().ends_with("json") {
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
