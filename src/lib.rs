//! # Panbuild
//!
//! `panbuild` is the universal builder.
use std::collections::HashMap;

mod execution_context;
mod manifests;
mod projects;
mod utils;

pub use manifests::manifest::AbstractManifest;
pub use manifests::manifest::AbstractModule;

use std::env;
use std::fs;
use std::path;

const DEFAULT_CACHE_DIR: &str = ".panbuild/";
const DEFAULT_GIT_CACHE_DIR: &str = ".git/";
const DEFAULT_FLATPAK_BUILDER_CACHE_DIR: &str = ".flatpak-builder/";
const DEFAULT_FLATPAK_BUILD_CACHE_DIR: &str = ".build/";
const DEFAULT_PACKAGE_LIST_SEP: &str = ",";

struct PanbuilbArguments {
    // TODO use enum for command name?
    command_name: String,
    arguments: Vec<String>,
    // TODO use enums for those?
    input_format: String,
    output_format: String,
}

pub fn run(command_name: &str, args: HashMap<String, String>) -> i32 {
    // FIXME put to debug once there is proper logging in place
    // eprintln!("running command {}.", command_name);
    let mut ctx = crate::execution_context::ExecutionContext::default();

    if command_name == "lint" {
        let input_file_path = match args.get("input_file") {
            Some(input_file_path) => input_file_path,
            None => {
                eprintln!("an input file is required!");
                // TODO handle reading from stdin.
                return 1;
            }
        };

        ctx.content = match fs::read_to_string(path::Path::new(input_file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", input_file_path);
                return 1;
            }
        };

        if args.contains_key("input_format") {
            let source_type = args.get("input_format").unwrap();
            if !crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        } else {
            let mut exit_code: i32 = manifests::detect_type(&mut ctx);
            if exit_code != 0 {
                eprintln!("Could not detect manifest type of {}.", ctx.source_filename);
                return exit_code;
            }
        }

        let mut exit_code: i32 = manifests::parse(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while parsing");
            return exit_code;
        }

        eprintln!("Parsing finished. Resulting manifest is {:#?}", &ctx.manifest);

        exit_code = manifests::dump(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while dumping");
            return exit_code;
        }

        match fs::write(path::Path::new(input_file_path), ctx.content) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not write file {}.", input_file_path);
                return 1;
            }
        };

        eprintln!("Dumped the manifest!");
        return 0;
    }

    if command_name == "get-package-list" {
        let input_file_path = match args.get("input_file") {
            Some(input_file_path) => input_file_path,
            None => {
                eprintln!("an input file is required!");
                // TODO handle reading from stdin.
                return 1;
            }
        };

        ctx.content = match fs::read_to_string(path::Path::new(input_file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", input_file_path);
                return 1;
            }
        };

        if args.contains_key("input_format") {
            let source_type = args.get("input_format").unwrap();
            if !crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        } else {
            let mut exit_code: i32 = manifests::detect_type(&mut ctx);
            if exit_code != 0 {
                eprintln!("Could not detect manifest type of {}.", ctx.source_filename);
                return exit_code;
            }
        }

        let mut exit_code: i32 = manifests::parse(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while parsing");
            return exit_code;
        }

        eprintln!("Parsing finished. Resulting manifest is {:#?}", &ctx.manifest);

        let mut separator = DEFAULT_PACKAGE_LIST_SEP;
        if args.contains_key("separator") {
            separator = args.get("separator").unwrap();
        }

        exit_code = manifests::get_modules(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while getting modules");
            return exit_code;
        }

        let mut output: String = String::from("");
        for module in &ctx.manifest.depends_on {
            if !output.is_empty() {
                output.push_str(&separator)
            }
            output.push_str(&module.name);
        }
        println!("{}", output);
    }

    // Used for debugging. Show all the available projects.
    if command_name == "projects" {
        let projects: Vec<crate::projects::project::Project> = crate::projects::db::get_all();
        for project in projects {
            println!("{0}: {1}", project.name, project.summary);
        }
        return 0;
    }

    if command_name == "install" {
        let input_file_path = match args.get("input_file") {
            Some(input_file_path) => input_file_path,
            None => {
                eprintln!("an input file is required!");
                // TODO handle reading from stdin.
                return 1;
            }
        };

        ctx.content = match fs::read_to_string(path::Path::new(input_file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", input_file_path);
                return 1;
            }
        };

        if args.contains_key("input_format") {
            let source_type = args.get("input_format").unwrap();
            if !crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        } else {
            let mut exit_code: i32 = manifests::detect_type(&mut ctx);
            if exit_code != 0 {
                eprintln!("Could not detect manifest type of {}.", ctx.source_filename);
                return exit_code;
            }
        }

        let mut exit_code: i32 = manifests::parse(&mut ctx);
        if exit_code != 0 {
            eprintln!("Error while parsing");
            return exit_code;
        }

        eprintln!("Parsing finished. Resulting manifest is {:#?}", &ctx.manifest);

        let config = match crate::execution_context::read_or_init_config() {
            Ok(c) => c,
            Err(e) => panic!("Could not load or init config: {}", e),
        };

        let package_name = match args.get("package_name") {
            Some(package_name) => package_name,
            None => {
                eprintln!("A package name to install is required!");
                return 1;
            }
        };
        if package_name.len() < 3 {
            eprintln!("{} is too short for a package name!", package_name);
            return 1;
        }
        eprintln!("Installing module {:#?}", &package_name);

        let packages: Vec<crate::manifests::manifest::AbstractModule> = crate::projects::get_modules();
        eprintln!("Searching in {:#?} packages for installation candidates 🕰", packages.len());
        for package in &packages {
            if package.name.contains(package_name) {
                println!("found candidate artifact in {}.", package.name);
            }
        }

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
        for path in file_paths.iter() {
            let file_path = path;
            let file_path_str = file_path.to_str().unwrap();
            if file_path.is_dir() {
                continue;
            }

            // TODO Test that if it starts with the cache directories listed above,
            // you skip the file.

            if crate::manifests::debian::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("debian ({})", file_path_str);
            }
            if crate::manifests::snap::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("snap ({})", file_path_str);
            }
            if crate::manifests::flatpak::file_path_matches(file_path_str) {
                found_manifest = true;
                println!("flatpak ({})", file_path_str);
            }
        }

        if !found_manifest {
            eprintln!("No available workspace found for the project. Try running `ls -p`.");
        } else {
            println!("Use `checkout` to select a workspace.");
        }
    }

    if command_name == "checkout" {
        let mut config = match crate::execution_context::read_or_init_config() {
            Ok(c) => c,
            Err(e) => panic!("Could not load or init config: {}", e),
        };

        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        config.current_workspace = Some(env_name.to_string());
        match crate::execution_context::write_config(&config) {
            Ok(c) => c,
            Err(e) => panic!("Could not write config: {}", e),
        };
    }

    if command_name == "create" {
        let mut config = match crate::execution_context::read_or_init_config() {
            Ok(c) => c,
            Err(e) => panic!("Could not load or init config: {}", e),
        };

        let env_name = match args.get("env_name") {
            Some(n) => n,
            None => panic!("An env name is required to checkout."),
        };

        if *env_name == config.current_workspace.unwrap_or("".to_string()) {
            println!("Already in workspace {}.", env_name);
            return 0;
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

        println!("🗃 Created workspace {} with manifest file {}.", env_name, manifest_file_path);
    }

    if command_name == "status" {
        let mut config = match crate::execution_context::read_or_init_config() {
            Ok(c) => c,
            Err(e) => panic!("Could not load or init config: {}", e),
        };

        match config.current_workspace {
            Some(workspace) => println!("Current workspace is {}.", workspace),
            None => println!("No current workspace. Call `ls` to list the available manifests."),
        };
    }

    // FIXME put to debug once there is proper logging in place
    // eprintln!("Finishing...");
    return 0;
}
