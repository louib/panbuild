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

    if command_name == "parse" {
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

        let mut output: String = String::from("");
        // FIXME we should fetch those recursively.
        for module in &ctx.manifest.depends_on {
            // FIXME should we check for duplicates here??
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

        let package_name  = match args.get("package_name") {
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
        eprintln!("Installing {:#?}", &package_name);

        let projects: Vec<crate::projects::project::Project> = crate::projects::db::get_all();
        eprintln!("Searching in {:#?} projects for installation candidates 🕰",projects.len());
        for project in &projects {
            for artifact_name in &project.artifact_names {
                if artifact_name.contains(package_name) {
                    println!("found candidate artifact in {}.", project.name);
                }
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

        let mut found_env = false;
        let file_paths = fs::read_dir("./").unwrap();
        for path in file_paths {
            let file_path = path.unwrap().path();
            let file_path = path::Path::new(&file_path);
            let file_path_str = file_path.to_str().unwrap();
            if file_path.is_dir() {
                continue;
            }

            // TODO Test that if it starts with the cache directories listed above,
            // you skip the file.

            if crate::manifests::debian::file_path_matches(file_path_str) {
                found_env = true;
                println!("debian ({})", file_path_str);
            }
            if crate::manifests::snap::file_path_matches(file_path_str) {
                found_env = true;
                println!("snap ({})", file_path_str);
            }
            if crate::manifests::flatpak::file_path_matches(file_path_str) {
                found_env = true;
                println!("flatpak ({})", file_path_str);
            }
            println!("{}", file_path_str);
        }

        if !found_env {
            eprintln!("No available environment found for the project. Try running `ls -p`.");
        }
    }

    if command_name == "status" {
        let project_path = path::Path::new(DEFAULT_CACHE_DIR);
        if !project_path.is_dir() {
            println!("No environment configured yet. Run `ls` to show the available envs.");
        }
    }

    // FIXME put to debug once there is proper logging in place
    // eprintln!("Finishing...");
    return 0;
}
