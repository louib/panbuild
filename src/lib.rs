// FIXME this dependency should be removed.
use clap::{ArgMatches};

mod manifest;

use std::fs;
use std::path;

pub fn run(command_name: &str, args: &ArgMatches) -> i32 {
    println!("running command {}.", command_name);

    if command_name == "convert" {
        if ! args.is_present("input_file") {
            println!("an input file is required when converting!");
            // TODO handle reading from stdin.
            return 1;
        }

        let input_file = args.value_of("input_file").unwrap();
        let input_file_path = path::Path::new(input_file);

        let fs_read_result = fs::read_to_string(input_file_path);
        if fs_read_result.is_err() {
            println!("could not read file {}.", input_file);
            return 1;
        }

        let manifest_content = fs_read_result.unwrap();

        let manifest_type: &str = "snap";
        manifest::get_type(input_file.to_string(), manifest_type);

        let ctx = manifest::ConversionContext {
          source_type: "snap".to_string(),
          destination_type: "flatpak".to_string(),
          // content: manifest_content,
          content: "".to_string(),
        };

        let manifest = manifest::parse(manifest_content, manifest_type.to_string(), &ctx);
        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    return 0;
}
