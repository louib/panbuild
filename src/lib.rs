// FIXME this dependency should be removed.
use clap::{ArgMatches};

mod manifests;
mod execution_context;
mod utils;

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

        let mut ctx = crate::execution_context::ExecutionContext::default();
        ctx.content = fs_read_result.unwrap();

        let mut exit_code: i32 = manifests::get_type(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::parse(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        exit_code = manifests::dump(&ctx);
        if exit_code != 0 {
            return exit_code;
        }

        return 0;
    }

    if command_name == "ls" {
        return 0;
    }

    return 0;
}
