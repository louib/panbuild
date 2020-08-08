//! # Panbuild
//!
//! `panbuild` is the universal build manifest converter.
use std::collections::HashMap;

mod manifests;
mod projects;
mod execution_context;
mod utils;

pub use manifests::manifest::AbstractManifest;
pub use manifests::manifest::AbstractModule;

use std::fs;
use std::path;
use std::env;

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
    eprintln!("running command {}.", command_name);
    let mut ctx = crate::execution_context::ExecutionContext::default();

    if command_name == "convert" {
        let input_file_path = match args.get("input_file") {
            Some(input_file_path) => input_file_path,
            None => {
                eprintln!("an input file is required when converting!");
                // TODO handle reading from stdin.
                return 1;
            },
        };

        ctx.content = match fs::read_to_string(path::Path::new(input_file_path)) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("could not read file {}.", input_file_path);
                return 1;
            }

        };

        ctx.data_dir = env::var("PANBUILD_DATA_DIR").unwrap_or(String::from("")).to_string();

        if args.contains_key("input_format") {
            let source_type = args.get("input_format").unwrap();
            if ! crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        }

        if args.contains_key("output_format") {
            let destination_type = args.get("output_format").unwrap();
            if ! crate::manifests::has_type(destination_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", destination_type);
                return 1;
            }
            ctx.destination_type = destination_type.to_string();
        }

        let mut exit_code: i32 = manifests::get_type(&mut ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::parse(&mut ctx);
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

        eprintln!("Finishing...");
        return 0;
    }

    if command_name == "get-package-list" {
        let input_file_path = match args.get("input_file") {
            Some(input_file_path) => input_file_path,
            None => {
                eprintln!("an input file is required when converting!");
                // TODO handle reading from stdin.
                return 1;
            },
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
            if ! crate::manifests::has_type(source_type.to_string()) {
                eprintln!("{} is an invalid manifest type.", source_type);
                return 1;
            }
            ctx.source_type = source_type.to_string();
        }

        let mut exit_code: i32 = manifests::get_type(&mut ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::parse(&mut ctx);
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
        for module in ctx.manifest.depends_on {
            // FIXME should we check for duplicates here??
            if ! output.is_empty() {
                output.push_str(&separator)
            }
            output.push_str(&module.name);
        }
        println!("{}", output);
    }

    if command_name == "ls" {
        let projects: Vec<crate::projects::project::Project> = crate::projects::db::get_all();
        for project in projects {
            println!("{0}: {1}", project.name, project.summary);
        }
        return 0;
    }

    eprintln!("Finishing...");
    return 0;
}
