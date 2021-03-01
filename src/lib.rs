//! # Panbuild
//!
//! `panbuild` is the universal builder.
use std::collections::HashMap;

pub mod db;
pub mod hubs;
pub mod logger;
pub mod manifests;
pub mod modules;
pub mod utils;
pub mod projects;

mod config;
mod developers;
mod version;

pub use manifests::manifest::AbstractManifest;
pub use modules::SoftwareModule;
pub use projects::SoftwareProject;

use std::env;
use std::fs;
use std::path;

const DEFAULT_GIT_CACHE_DIR: &str = ".git/";
const DEFAULT_PACKAGE_LIST_SEP: &str = ",";

struct PanbuilbArguments {
    // TODO use enum for command name?
    command_name: String,
    arguments: Vec<String>,
    // TODO use enums for those?
    output_format: String,
}

pub fn run(command_name: &str, args: HashMap<String, String>) -> i32 {
    logger::init();

    log::debug!("running command {}.", command_name);

    let mut config = match crate::config::read_or_init_config() {
        Ok(c) => c,
        Err(e) => panic!("Could not load or init config: {}", e),
    };

    if command_name == "lint" {
        let manifest_file_path = args.get("manifest_file_path").expect("an input file is required!");

        let mut abstract_manifest = match crate::manifests::manifest::AbstractManifest::load_from_file(manifest_file_path.to_string()) {
            Some(m) => m,
            None => return 1,
        };

        let manifest_dump = match abstract_manifest.dump() {
            Ok(d) => d,
            Err(e) => return 1,
        };

        match fs::write(path::Path::new(manifest_file_path), manifest_dump) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write file {}.", manifest_file_path);
                return 1;
            }
        };

        eprintln!("Dumped the manifest!");
        return 0;
    }

    if command_name == "get-package-list" {
        let manifest_file_path = args.get("manifest_file_path").expect("a manifest file is required!");

        let mut abstract_manifest = match crate::manifests::manifest::AbstractManifest::load_from_file(manifest_file_path.to_string()) {
            Some(m) => m,
            None => return 1,
        };

        let mut separator = DEFAULT_PACKAGE_LIST_SEP;
        if args.contains_key("separator") {
            separator = args.get("separator").unwrap();
        }

        let modules = match abstract_manifest.get_modules() {
            Ok(m) => m,
            Err(m) => return 1,
        };

        let mut output: String = String::from("");
        for module in &modules {
            if !output.is_empty() {
                output.push_str(&separator)
            }
            output.push_str(&module.name);
        }
        println!("{}", output);
    }

    if command_name == "search" {
        let search_term = match args.get("search_term") {
            Some(search_term) => search_term,
            None => {
                eprintln!("A search term is required!");
                return 1;
            }
        };
        if search_term.len() < 3 {
            eprintln!("{} is too short for a search term!", search_term);
            return 1;
        }
        eprintln!("Search for {} in the projects database.", &search_term);

        let db = crate::db::Database::get_database();
        let modules: Vec<&SoftwareModule> = db.search_modules(search_term);
        for module in modules {
            println!("found candidate artifact in {}.", module.name);
        }
        let projects: Vec<&SoftwareProject> = db.search_projects(search_term);
        for project in projects {
            println!("found candidate artifact in {}.", project.name);
        }
    }

    if command_name == "install" {
        let mut abstract_manifest = match crate::config::load_manifest_from_config() {
            Some(m) => m,
            None => return 1,
        };

        let package_name = args.get("package_name").expect("A package name to install is required!");
        if package_name.len() < 3 {
            eprintln!("{} is too short for a package name!", package_name);
            return 1;
        }
        eprintln!("Installing module {:#?}", &package_name);

        let packages: Vec<SoftwareModule> = crate::projects::get_modules();
        let mut installed_package: Option<&SoftwareModule> = None;
        eprintln!("Searching in {:#?} packages for installation candidates 🕰", packages.len());
        for package in &packages {
            if package.name.contains(package_name) {
                println!("found candidate artifact in {}.", package.name);
                let question = format!("Do you want to install {} ({})", package.name, package.url);
                if crate::utils::ask_yes_no_question(question) {
                    println!("installing {}.", package.name);
                    abstract_manifest.add_module(package);
                    installed_package = Some(package);
                    break;
                }
            }
        }

        let installed_package_name = match installed_package {
            Some(p) => p,
            None => {
                println!("Did not install any package.");
                return 1;
            }
        };
        let installed_package_name = &installed_package_name.name;
        println!("Installed package {}.", installed_package_name);

        let manifest_dump = match abstract_manifest.dump() {
            Ok(d) => d,
            Err(e) => return 1,
        };

        match fs::write(path::Path::new(&abstract_manifest.path), manifest_dump) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write file {}.", &abstract_manifest.path);
                return 1;
            }
        };
        return 0;
    }

    if command_name == "make" {
        let mut abstract_manifest = match crate::config::load_manifest_from_config() {
            Some(m) => m,
            None => return 1,
        };

        match abstract_manifest.run_build() {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not run build for manifest file {}: {}", &abstract_manifest.path, e);
                return 1;
            }
        };
        // TODO report on the build!
        return 0;
    }

    if command_name == "run" {
        let mut abstract_manifest = match crate::config::load_manifest_from_config() {
            Some(m) => m,
            None => return 1,
        };

        let command = match args.get("command") {
            Some(n) => n,
            None => panic!("A command to run is required."),
        };
        println!("Running command `{}` in workspace {}", command, config.current_workspace.unwrap());

        match abstract_manifest.run_command(command) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not run build for manifest file {}: {}", &abstract_manifest.path, e);
                return 1;
            }
        };
        return 0;
    }

    if command_name == "ls" {
        let git_cache_dir = path::Path::new(DEFAULT_GIT_CACHE_DIR);
        if !git_cache_dir.is_dir() {
            eprintln!("This does not seem like a git project (.git/ was not found).");
            return 1;
        }

        let mut found_manifest = false;
        let file_paths = match utils::get_all_paths(path::Path::new("./")) {
            Ok(paths) => paths,
            Err(message) => {
                eprintln!("Could not get the file paths :sad: {}", message);
                return 1;
            }
        };
        // TODO print also those already matched to workspaces.
        for path in file_paths.iter() {
            let file_path = path;
            let file_path_str = file_path.to_str().unwrap();
            if !file_path.is_file() {
                continue;
            }
            // TODO Test that if it starts with the cache directories listed above,
            // you skip the file.

            if let Some(manifest) = crate::manifests::manifest::AbstractManifest::load_from_file(file_path_str.to_string()) {
                println!("{} {}", manifest.get_type().unwrap_or("unknown"), file_path_str);
                found_manifest = true;
            }
        }

        if !found_manifest {
            eprintln!("No available workspace found for the project. Try running `ls -p`.");
        } else {
            println!("Use `checkout` to select a workspace.");
        }
    }

    if command_name == "checkout" {
        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        if let Some(current_workspace) = &config.current_workspace {
            if current_workspace == env_name {
                println!("Already in workspace {}.", env_name);
                return 0;
            }
        }

        if !config.workspaces.contains_key(env_name) {
            eprintln!(
                "Workspace {} does not exist. Use `ls` to list the available workspaces and manifests.",
                env_name
            );
            return 1;
        }

        config.current_workspace = Some(env_name.to_string());
        match crate::config::write_config(&config) {
            Ok(c) => c,
            Err(e) => panic!("Could not write config: {}", e),
        };
    }

    if command_name == "create" {
        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        if let Some(current_workspace) = &config.current_workspace {
            if current_workspace == env_name {
                println!("Already in workspace {}.", env_name);
                return 0;
            }
        }

        if config.workspaces.contains_key(env_name) {
            eprintln!("Workspace {} already exists.", env_name);
            return 1;
        }

        let manifest_file_path = match args.get("manifest_file_path") {
            Some(p) => p,
            None => {
                eprintln!("a manifest file is required to create a new workspace!");
                // TODO handle reading from stdin.
                return 1;
            }
        };

        config.workspaces.insert(env_name.to_string(), manifest_file_path.to_string());
        config.current_workspace = Some(env_name.to_string());
        match crate::config::write_config(&config) {
            Ok(c) => c,
            Err(e) => panic!("Could not write config: {}", e),
        };
        println!("🗃 Created workspace {} with manifest file {}.", env_name, manifest_file_path);
    }

    if command_name == "status" {
        let current_workspace = match config.current_workspace {
            Some(workspace) => workspace,
            None => "".to_string(),
        };

        if current_workspace.len() == 0 {
            println!("Not in a workspace. Call `ls` to list the workspaces and manifest files.");
            return 0;
        }

        if !config.workspaces.contains_key(&current_workspace) {
            panic!("Workspace {} not found in config!.", current_workspace);
            return 1;
        }

        let manifest_file_path = config.workspaces.get(&current_workspace).unwrap();
        println!("Workspace {} using {}.", current_workspace, manifest_file_path);
    }

    log::debug!("Finishing...");
    return 0;
}
