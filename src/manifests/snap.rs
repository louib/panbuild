use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// See https://snapcraft.io/docs/snapcraft-yaml-reference for the full YAML reference.
// TODO is https://snapcraft.io/docs/snapcraft-advanced-grammar relevant?
// The top-level keys and values in snapcraft.yaml provide the snap build process, and the store,
// with the overarching details of a snap. See Snapcraft app metadata and Snapcraft parts metadata for
// details on how apps and parts are configured within snapcraft.yaml.
// Top-level details include a snap’s name, version and description, alongside operational values
// such as its confinement level and supported architecture.
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct SnapcraftManifest {
    // Incorporate external metadata via the referenced part.
    // See Using external metadata for more details.
    pub adopt_info: String,

    // List of build and run architectures.
    // For more details, see https://snapcraft.io/docs/architectures
    pub architectures: Vec<String>,

    // A list of features that must be supported by the core in order for this snap to install.
    //
    // For example to make the snap only installable on certain recent
    // version of snapd(like 2.38) you can specify:
    //   assumes:
    //   - snapd2.38
    //
    // Other potential values for assumes include:
    //   common-data-dir: support for common data directory across revisions of a snap.
    //   snap-env: support for the “Environment:” feature in snap.yaml
    //   command-chain: support for the “command-chain” feature for apps and hooks in snap.yaml
    //
    // (optional)
    pub assumes: Vec<String>,

    // A snap of type base to be used as the execution environment for this snap.
    // See https://snapcraft.io/docs/base-snaps for further details.
    //
    // Values:
    //   bare	Empty base snap, useful for fully statically linked snaps and testing
    //   core	Ubuntu Core 16
    //   core18	Ubuntu Core 18
    //   core20	Ubuntu Core 20
    pub base: String,

    // Determines if the snap should be restricted in access or not.
    pub confinement: String,

    // Multi-line description of the snap.
    //
    // A more in-depth look at what your snap does and who may find it most useful.
    pub description: String,

    // Defines the quality grade of the snap.
    pub grade: String,

    // Path to icon image that represents the snap in the snapcraft.io
    // store pages and other graphical store fronts.
    //
    // Note that the desktop menu does not use this icon.
    // It uses the icon in the .desktop file of the application.
    //
    // It is a relative path to a .png/.svg file from the source tree root.
    // The recommended size is 256x256 pixels.
    // Aspect ratio needs to be 1:1. Image size can vary from 40x40 to 512x512 px and
    // the file size should not be larger than 256 KB.
    //
    // Examples: _package_name_.svg, or snap/gui/logo.png
    pub icon: String,

    // A license for the snap in the form of an SPDX expression for the license.
    //
    // In the legacy Snapcraft syntax (not using the base key), this key is only
    // available through the passthrough key.
    //
    // Currently, only SPDX 2.1 expressions are supported. A list of supported values
    // are also available at snapd/licenses.go at master · snapcore/snapd.
    //
    // For “or later” and “with exception” license styles refer to the Appendix IV
    // of the SPDX Specification 2.1.
    //
    // Examples: GPL-3.0+, MIT, Proprietary
    pub license: String,

    // The identifying name of the snap.
    //
    // It must start with an ASCII character and can only contain letters in lower case, numbers, and
    // hyphens, and it can’t start or end with a hyphen.
    // The name must be unique if you want to publish to the Snap Store.
    //
    // For help on choosing a name and registering it on the Snap Store, see Registering your app name.
    //
    // Example: my-awesome-app
    pub name: String,

    // Sentence summarising the snap.
    //
    // Max len. 78 characters, describing the snap in short and simple terms.
    //
    //   Example: The super cat generator
    pub summary: String,

    // The canonical title of the application, displayed in the software centre graphical frontends.
    //
    // Max length 40 characters.
    //
    // In the legacy Snapcraft syntax (not using the base key),
    // this key is only available through the passthrough key.
    pub title: String,

    // The type of snap, implicitly set to app if not set.
    //
    // For more details, see:
    //    https://snapcraft.io/docs/gadget-snap,
    //    https://snapcraft.io/docs/kernel-snap,
    //    https://snapcraft.io/docs/base-snaps,
    pub r#type: String,

    // A user facing version to display.
    //
    // Max len. 32 chars. Needs to be wrapped with single-quotes when the value will be
    // interpreted by the YAML parser as non-string.
    //
    // Examples: '1', '1.2', '1.2.3', git (will be replaced by a git describe based version string)
    pub version: String,

    pub default_version: String,

    // Plugs and slots for an interface are usually configured per-app or per-daemon within snapcraft.yaml.
    // See https://snapcraft.io/docs/snapcraft-app-and-service-metadata for more details.
    // However, snapcraft.yaml also enables global plugs and slots configuration for an entire snap
    pub plugs: HashMap<String, SnapcraftPlug>,
    // pub slots: HashMap<String, SnapcraftSlot>,

    // A map of app-names representing entry points to run for the snap.
    pub apps: HashMap<String, SnapcraftApp>,

    pub parts: HashMap<String, SnapcraftPart>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct SnapcraftPlug {
    pub interface: String,
    pub target: String,
    pub default_provider: String,
}

const REQUIRED_TOP_LEVEL_FIELDS: [&'static str; 0] = [
    // DESCRIPTION,
    // GRADE,
    // NAME,
    // SUMMARY,
    // According to the spec, the version is indeed required, but some projects don't
    // populate it so it can be generated dynamically when creating a new version.
    // See the `snapcraftctl set-version` command for examples.
    // VERSION,
];

// strict: no access outside of declared interfaces through plugs.
// devmode: a special mode for snap creators and developers.
// classic: allows access to your system’s resources in much the same way traditional packages do.
// For more information, refer to https://snapcraft.io/docs/snap-confinement.
enum Confinement {
    Strict,
    Devmode,
    Classic,
}

// devel (i.e. a development version of the snap, so not to be published to the stable or candidate channels).
// stable (i.e. a stable release or release candidate, which can be released to all channels).
enum Grade {
    Stable,
    Devel,
}

// The app keys and values in snapcraft.yaml detail the applications and services that
// a snap wants to expose, including how they’re executed and which resources they can access.
// See Snapcraft top-level metadata and Snapcraft parts metadata for details on
// how apps and parts are configured within snapcraft.yaml.
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct SnapcraftApp {
    // Can be one of the following:
    //   none (Disables the creation of an env variable wrapper.)
    //   full (default)
    // Snapcraft normally creates a wrapper holding common environment variables.
    // Disabling this could be useful for minimal base snaps without a shell,
    // and for statically linked binaries with no use for an environment.
    pub adapter: String,

    // Defines the name of the .desktop file used to start an application with the desktop session.
    // The desktop file is placed in $SNAP_USER_DATA/.config/autostart,
    // and the application is started using the app’s command wrapper (<name>.<app>)
    // plus any argument present in the Exec= line within the .desktop file.
    //
    //   Example: autostart: my-chat.desktop
    // See Autostart desktop files for an example of both the desktop file and the Exec file entry.
    pub autostart: String,

    // The command to run inside the snap when <app-name> is invoked.
    // The command can be in either a snap runtime’s command path,
    // $SNAP/usr/sbin:$SNAP/usr/bin:$SNAP/sbin:$SNAP/bin, or an executable path relative to $SNAP.
    //
    // If daemon is set, this will be the command to run the service.
    // Only a snap with classic confinement can use a relative path because PATH isn’t
    // modified by a wrapper in classic confinement.
    // See Classic confinement for more details.
    //   Examples: app-launch for an excecutable placed under $SNAP/bin.
    //   With classic confinement, bin/app-launch for an executable placed under $SNAP/bin.
    pub command: String,

    // A list of command to be executed, in order, before the command referenced by apps.<app-name>.command.
    //   See Proposal: support command-chain in apps and hooks for further details.
    // To ensure that the Snapd distribution user running supports this feature,
    // add the command-chain value to the assumes property.
    pub command_chain: Vec<String>,

    // An identifier to a desktop-id within an external appstream file.
    // See Using external metadata for more details.
    pub common_id: String,

    // Declares that <app-name> is a system daemon.
    pub daemon: String,

    // Location of the .desktop file.
    // A path relative to the prime directory pointing to a desktop file,
    // commonly used to add an application to the launch menu. Snapcraft will take care of the rest.
    //   Examples: usr/share/applications/my-app.desktop and share/applications/my-app.desktop
    pub desktop: String,

    // A set of key-value pairs specifying the contents of environment variables.
    // Key is the environment variable name; Value is the contents of the environment variable.
    //   Example: LANG: C.UTF-8
    pub environment: HashMap<String, String>,

    // Extensions to apply to this application.
    //   Example: [gnome-3-28]
    pub extensions: Vec<String>,

    // <app-name> attributes to pass through to snap.yaml without snapcraft validation.
    // See Using in-development features for further details.
    // const PASSTHROUGH: &str = "passthrough";
    // Plugs for interfaces to connect to.
    // <app-name> will make these plug connections when running in strict confinement.
    // For interfaces that need attributes, see top-level plugs.
    //   Example: [home, removable-media, raw-usb]
    pub plugs: Vec<String>,

    // Runs a command from inside the snap after a service stops.
    // Requires daemon to be set as the snap type.
    pub post_stop_command: String,

    // Condition to restart the daemon under.
    // Requires daemon to be set as the snap type.
    pub restart_condition: String,

    // Slots for interfaces to connect to.
    // <app-name> will make these slot connections when running in strict confinement only.
    // For interfaces that need attributes, see top-level slots.
    //   Example: [home, removable-media, raw-usb]
    pub slots: Vec<String>,

    // Type: dict
    // Maps a daemon’s sockets to services and activates them.
    // Requires an activated daemon socket.
    // Requires apps.<app-name>.plugs to declare the network-bind plug.
    pub socket: HashMap<String, SnapcraftSocket>,

    // The path to a command inside the snap to run to stop the service.
    // Requires daemon to be set as the snap type.
    pub stop_command: String,

    // The length of time to wait before terminating a service.
    // Time duration units can be 10ns, 10us, 10ms, 10s, 10m.
    // Termination is via SIGTERM (and SIGKILL if that doesn’t work).
    // Requires daemon to be set as the snap type.
    pub stop_timeout: String,

    // Schedules when, or how often, to run a service or command.
    // See Timer string format for further details on the required syntax.
    // Requires daemon to be set as the snap type.
    pub timer: String,
}

// Sockets should go to a map of <socket-name>\ to objects which specify the
// listen-stream and (optionally) the socket-mode.
//
// TCP socket syntax: <port>, [::]:<port>, [::1]:<port> and 127.0.0.1:<port>
// UNIX socket syntax: $SNAP_DATA/<path>, $SNAP_COMMON/<path> and @snap.<snap name>.<suffix>
//
// Example:
//     unix:
//       listen-stream: $SNAP_COMMON/lxd/unix.socket
//       socket-mode: 0660
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct SnapcraftSocket {
    pub listen_stream: String,

    // The mode of a socket in octal.
    // FIXME change to a integer in octal mode.
    pub socket_mode: String,
}

// Refer to systemd.service manual for details.
pub const ALLOWED_RESTARTS_CONDITIONS: [&str; 6] = ["on_failure", "on_success", "on_abnormal", "on_abort", "always", "never"];

pub const ALLOWED_DAEMON_TYPES: [&str; 4] = [
    // the command is the main process.
    "simple",  // the configured command will exit after completion
    "oneshot", // the configured command calls fork() as part of its start-up.
    // The parent process is then expected to exit when start-up is complete
    "forking", // the command configured will send a signal to systemd to indicate that it’s running.
    "notify",
];

// For more information, refer to the output of snapcraft help plugins .
pub const ALLOWED_BUILD_ATTRIBUTES: [&str; 4] = [
    // Plugins that support the concept of build types build in Release mode by default.
    // Setting the ‘debug’ attribute requests that they instead build in debug mode.
    "debug",
    // Do not remove the “executable stack” bit from ELF files.
    "keep_execstack",
    // Do not patch ELF files.
    "no_patchelf",
    // Do not run the install target provided by the plugin’s build system.
    // (Only supported by the kbuild plugin)
    "no_install",
];

// The main building blocks of a snap are parts.
// They are used to declare pieces of code that will be pulled into your snap package.
// The parts keys and values in snapcraft.yaml detail how parts are configured and built
// by the snapcraft command.
//
// See Snapcraft top-level metadata and Snapcraft apps and services metadata for
// details on how apps and parts are configured within snapcraft.yaml.
// <part-name> represents the specific name of a building block which can be
// then referenced by the command line tool (i.e. snapcraft).
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct SnapcraftPart {
    // Ensures that all the <part-names> listed in after are staged before this part begins its lifecycle.
    pub after: Vec<String>,

    // A list of named attributes to modify the behaviour of plugins.
    pub build_attributes: Vec<String>,

    // A list of environment variable assignments that is applied during the build step,
    // it is exported in order which allows for later values to override (or modify) earlier values.
    pub build_environment: HashMap<String, String>,

    // A list of packages required to build a snap.
    //
    // Packages are installed using the host’s package manager, such as apt or dnf,
    // and are required for <part-name> to build correctly.
    // This entry supports additional syntax, for more information refer to Advanced grammar.
    //
    // Example: [ libssl-dev, libssh-dev, libncursesw5-dev]
    pub build_packages: Vec<String>,

    // A list of snap names to install that are necessary to build <part-name>.
    //
    // If a specific channel is required, the syntax is of the form <snap-name>/<channel>.
    // This entry supports additional syntax, for more information refer to Advanced grammar
    //
    // Example: build-snaps: [go/1.13/stable]
    pub build_snaps: Vec<String>,

    // A key to represent a group of files, or a single file.
    //
    // See Snapcraft filesets for further details.
    pub filesets: Vec<String>,

    // Runs a script after the plugin’s build step.
    //
    // The shell script defined here is run after the build step of the
    // plugin defined in parts.<part-name>.plugin starts.
    // The working directory is the base build directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // The release of Snapcraft 3.0 made this key obsolete. Use override-build instead.
    pub install: String,

    // A map of files to rename.
    //
    // In the key/value pair, the key represents the path of a file inside the part
    // and the value represents how the file is going to be staged.
    //
    // Example: bin/snapcraftctl: bin/scriptlet-bin/snapcraftctl
    pub organize: HashMap<String, String>,

    // Replaces a plugin’s default build process with a script.
    //
    // The shell script defined here replaces the build step of the plugin,
    // defined in parts.<part-name>.plugin.
    //
    // The working directory is the base build directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // To run Snapcraft’s original build implementation from within override-build, run snapcraftctl build.
    // This can be run before or after any custom script, or omitted entirely.
    pub override_build: String,

    // Replaces a plugin’s default prime process with a script.
    //
    // The shell script defined here replaces the prime step of the plugin,
    // defined in parts.<part-name>.plugin.
    // The working directory is the base prime directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // To run Snapcraft’s original prime step implementation from within override-prime,
    // run snapcraftctl prime.
    // This can be run before or after any custom script, or omitted entirely.
    pub override_prime: String,

    // Replaces a plugin’s default pull process with a script.
    //
    // The shell script defined here replaces the pull step of the plugin,
    // defined in parts.<part-name>.plugin.
    // The working directory is the base pull directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // To run Snapcraft’s original pull stage implementation from within override-pull, run snapcraftctl pull.
    // This can be run before or after any custom script, or omitted entirely.
    pub override_pull: String,

    // Replaces a plugin’s default stage process with a script.
    //
    // The shell script defined here replaces the stage step of the plugin,
    // defined in parts.<part-name>.plugin.
    // The working directory is the base stage directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // To run Snapcraft’s original stage implementation from within override-stage, run snapcraftctl stage.
    // This can be run before or after any custom script, or omitted entirely.
    pub override_stage: String,

    // Defines the content to adopt when using external metadata.
    //
    // It is a relative path to a supported metadata file from the part source,
    // build or install directory (SNAPCRAFT_PART_SRC, SNAPCRAFT_PART_BUILD, SNAPCRAFT_PART_INSTALL).
    //
    // See Using external metadata for more details.
    pub parse_info: String,

    // The plugin to drive the build process.
    //
    // Every part drives its build through a plugin, this entry declares the
    // plugin that will drive the build process for <part-name>.
    // Refer to snapcraft plugins for more information on the available plugins and the
    // specific attributes they add to the parts.<part-name>. namespace.
    // See https://snapcraft.io/docs/supported-plugins for the available plugins.
    pub plugin: String,

    // Runs a script before the plugin’s build step.
    //
    // The script is run before the build step defined for parts.<part-name>.plugin starts.
    // The working directory is the base build directory for the given part.
    // The defined script is run with /bin/sh and set -e.
    // A set of Environment Variables will be available to the script.
    //
    // The release of Snapcraft 3.0 made this key obsolete. Use override-build instead.
    pub prepare: String,

    // A list of files from <part-name> to prime.
    //
    // Rules applying to the list here are the same as those of filesets.
    // Referencing of fileset keys is done with a $ prefixing the fileset key,
    // which will expand with the value of such key.
    pub prime: Vec<String>,

    // A URL or path to a source tree to build.
    //
    // This can be a local path or remote, and can refer to a directory tree,
    // a compressed archive or a revision control repository.
    // This entry supports additional syntax, for more information refer to Advanced grammar
    pub source: String,

    // Work on a specific branch for source repositories under version control.
    pub source_branch: String,

    // Used when source represents a file.
    //
    // Takes the syntax <algorithm>/<digest>, where <algorithm> can be any of:
    //   md5,
    //   sha1,
    //   sha224,
    //   sha256,
    //   sha384,
    //   sha512,
    //   sha3_256,
    //   sha3_384,
    //   sha3_512.
    // When set, the source is cached for multiple uses in different snapcraft projects.
    pub source_checksum: String,

    // Work on a specific commit for source repositories under version control.
    pub source_commit: String,

    // Depth of history for sources using version control.
    //
    // Source repositories under version control are cloned or checked out with full history.
    // Specifying a depth will truncate the history to the specified number of commits.
    pub source_depth: i32,

    // A path within the source to set as the working directory when building.
    pub source_subdir: String,

    // Work on a specific tag for source repositories under version control.
    pub source_tag: String,

    // Used when the type of source entry cannot be detected.
    pub source_type: String,

    // A list of files from <part-name> to stage.
    //
    // Rules applying to the list here are the same as those of filesets.
    // Referencing of fileset keys is done with a $ prefixing the fileset key,
    // which will expand with the value of such key.
    pub stage: Vec<String>,

    // A list of packages required at runtime by a snap.
    //
    // Packages are required by <part-name> to run. They are fetched using the host’s package manager,
    // such as apt or dnf, and are unpacked into the snap being built.
    // This entry supports additional syntax, for more information refer to Advanced grammar.
    //
    // Example: [python-zope.interface, python-bcrypt]
    pub stage_packages: Vec<SnapcraftPackage>,

    // A list of snaps required at runtime by a snap.
    //
    // Snaps are required by <part-name> to run. They are fetched using snap download,
    // and are unpacked into the snap being built. This entry supports additional syntax,
    // for more information refer to Advanced grammar.
    //
    // Example: [hello, black/latest/edge]
    pub stage_snaps: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SnapcraftPackage {
    PackageName(String),
    OptionalPackages(SnapcraftOptionalPackages),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SnapcraftOptionalPackages {
    r#try: Vec<String>,
}

pub fn parse(ctx: &mut crate::execution_context::ExecutionContext) -> SnapcraftManifest {
    // TODO actually handle the error.
    let snapcraft_manifest: SnapcraftManifest = match serde_yaml::from_str(&ctx.content) {
        Ok(m) => m,
        Err(e) => panic!("Failed to parse the Snapcraft manifest: {}.", e),
    };

    // TODO I think there's other fields to validate here.
    if snapcraft_manifest.name.is_empty() {
        panic!("Required top-level field name is missing from snapcraft manifest.");
    }
    if snapcraft_manifest.grade.is_empty() {
        panic!("Required top-level field grade is missing from snapcraft manifest.");
    }

    return snapcraft_manifest;
}

pub fn dump(manifest: &crate::manifests::manifest::AbstractManifest) -> String {
    return String::from("");
}

pub fn file_path_matches(path: &str) -> bool {
    if path.to_lowercase().ends_with("snapcraft.yaml") {
        return true;
    }
    if path.to_lowercase().ends_with("snapcraft.yml") {
        return true;
    }
    return false;
}

pub fn file_content_matches(content: &str) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_file_path_matches() {
        assert!(file_path_matches("snapcraft.yaml"));
        assert!(file_path_matches("/path/to/snapcraft.yml"));
        assert!(file_path_matches("/path/to/Snapcraft.YAML"));
        assert!(!file_path_matches("/path/to/file.yaml"));
        assert!(!file_path_matches("/path/to/file.json"));
        assert!(!file_path_matches(""));
        assert!(!file_path_matches("/////////////"));
    }

    #[test]
    #[should_panic(expected = "Required top-level field grade is missing from snapcraft manifest.")]
    pub fn test_parse_missing_required_fields() {
        parse(
            r###"
            name: app-name,
            description: description
            summary: this is my app,
            version: 0.0.1
        "###,
        );
    }

    #[test]
    pub fn test_parse_missing_version() {
        let manifest: crate::manifests::manifest::AbstractManifest = parse(
            r###"
            name: app-name,
            description: description
            grade: devel
            summary: this is my app
        "###,
        );
        assert_eq!(manifest.package_version, "");
    }

    #[test]
    #[should_panic(expected = "Failed to parse the Snapcraft manifest: EOF while parsing a value.")]
    pub fn test_parse_empty_string() {
        parse("");
    }

    #[test]
    #[should_panic]
    pub fn test_parse_invalid_yaml() {
        parse("----------------------------");
    }
}
