use std::collections::HashMap;

// TODO tune built-in attributes
// From https://doc.rust-lang.org/reference/items/modules.html#attributes-on-modules
// The built-in attributes that have meaning on a module are cfg, deprecated, doc,
// the lint check attributes, path, and no_implicit_prelude.
// Modules also accept macro attributes.
#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::exit;

const APP_VERSION: &str = "0.0.11";

fn main() {
    let yaml = load_yaml!("panbuild.yml");
    let panbuild_app: App = App::from_yaml(yaml).version(APP_VERSION);

    // Here we could use get_matches_safe and override the error messages.
    // See https://docs.rs/clap/2.33.1/clap/struct.App.html#method.get_matches_safe
    let matches: ArgMatches = panbuild_app.get_matches();

    if matches.is_present("version") {
        println!("{}", APP_VERSION);
        exit(0);
    }

    let command_name = matches.subcommand_name().unwrap_or("");
    let mut flags: HashMap<String, bool> = HashMap::new();
    // let mut options: HashMap<String, bool> = HashMap::new();
    let mut arguments: HashMap<String, String> = HashMap::new();

    let command_name = match matches.subcommand_name() {
        Some(command_name) => command_name,
        None => {
            eprintln!("Please provide a command to execute.");
            eprintln!("{}", matches.usage());
            exit(1);
        }
    };

    let subcommand_matches = match matches.subcommand_matches(command_name) {
        Some(subcommand_matches) => subcommand_matches,
        None => {
            eprintln!("Invalid arguments for command {}", command_name);
            eprintln!("{}", matches.usage());
            exit(1);
        }
    };

    arguments
        .entry("input_format".to_string())
        .or_insert(subcommand_matches.value_of("input_format").unwrap_or("").to_string());
    arguments
        .entry("input_file".to_string())
        .or_insert(subcommand_matches.value_of("input_file").unwrap_or("").to_string());
    arguments
        .entry("manifest_file_path".to_string())
        .or_insert(subcommand_matches.value_of("manifest_file_path").unwrap_or("").to_string());
    arguments
        .entry("search_term".to_string())
        .or_insert(subcommand_matches.value_of("search_term").unwrap_or("").to_string());
    arguments.entry("separator".to_string()).or_insert(subcommand_matches.value_of("separator").unwrap_or(",").to_string());
    arguments
        .entry("package_name".to_string())
        .or_insert(subcommand_matches.value_of("package_name").unwrap_or("").to_string());
    arguments.entry("env_name".to_string()).or_insert(subcommand_matches.value_of("env_name").unwrap_or("").to_string());

    let exit_code = panbuild::run(command_name, arguments);
    exit(exit_code);
}
