// FIXME this dependency should be removed.
use clap::{ArgMatches};

mod manifest;

use std::fs;
use std::path;

pub fn run(command_name: &str, args: &ArgMatches) -> i32 {
    println!("running command {}.", command_name);

    if command_name == "convert" {
        if ! args.is_present("input_file") {
            // TODO handle reading from stdin.
            return 0;
        }

        let input_file = args.value_of("input_file").unwrap();
        let input_file_path = path::Path::new(input_file);

        let manifest_content = fs::read_to_string(input_file_path).unwrap();

        let manifest_type: &str = "snap";
        manifest::get_type(input_file.to_string(), manifest_type);

        let manifest = manifest::get_manifest(manifest_content, manifest_type.to_string());
        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    return 0;
}
