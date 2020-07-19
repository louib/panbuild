// FIXME this dependency should be removed.
use clap::{ArgMatches};

mod manifests;

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

        let input_file_path = args.value_of("input_file").unwrap();

        let fs_read_result = fs::read_to_string(path::Path::new(input_file_path));
        if fs_read_result.is_err() {
            println!("could not read file {}.", input_file_path);
            return 1;
        }

        let manifest_content = fs_read_result.unwrap();

        let ctx = manifests::manifest::ConversionContext {
            source_filename: input_file_path.to_string(),
            source_type: manifests::manifest::DEFAULT_SOURCE_TYPE.to_string(),
            destination_type: manifests::manifest::DEFAULT_DESTINATION_TYPE.to_string(),
            content: manifest_content,
            abstract_manifest: manifests::manifest::AbstractManifest::default(),
        };
        manifests::get_type(&ctx);

        manifests::parse(&ctx);
        manifests::dump(&ctx);
        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    return 0;
}
