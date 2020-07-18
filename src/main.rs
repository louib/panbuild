// TODO tune built-in attributes
// From https://doc.rust-lang.org/reference/items/modules.html#attributes-on-modules
// The built-in attributes that have meaning on a module are cfg, deprecated, doc,
// the lint check attributes, path, and no_implicit_prelude.
// Modules also accept macro attributes.
extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};
use std::process::{exit};

mod manifest;
mod snap;
mod npm;
mod flatpak;
mod debian;
mod pyproject;
mod utils;

fn main() {
    let pandoc_app: App = App::new("panbuild")
                          .version("0.0.1")
                          .author("louib <code@louib.net>")
                          .about("The universal build manifest converter.")
                          .arg(Arg::with_name("version")
                               .short("V")
                               .long("version")
                               .required(false)
                               .help("Show the version and exit."))
                          .subcommand(SubCommand::with_name("convert")
                               .about("convert a manifest file.")
                               .arg(Arg::with_name("input_file")
                                    .short("i")
                                    .long("input-file")
                                    .takes_value(true)
                                    .value_name("MANIFEST")
                                    .required(false)
                                    .help("Path of the input build manifest.")))
                          .subcommand(SubCommand::with_name("spec")
                               .about("Show the spec for a manifest type."));

    // Here we could use get_matches_safe and override the error messages.
    // See https://docs.rs/clap/2.33.1/clap/struct.App.html#method.get_matches_safe
    let matches: ArgMatches = pandoc_app.get_matches();

    if matches.is_present("version") {
        println!("0.0.1");
        exit(0);
    }

    let command_name = matches.subcommand_name().unwrap();

    match matches.subcommand_name() {
        Some(command_name)   => {
            if let Some(subcommand_matches) = matches.subcommand_matches(command_name) {
                let exit_code = panbuild::run(command_name, subcommand_matches);
                exit(exit_code);
            }
        },
        None => {
            // TODO this should print to stderr.
            println!("Please provide a command to execute.");
            exit(1);
        },
        _ => {
            // TODO this should print to stderr.
            println!("Unknown command {0}.", command_name);
            exit(1);
        },
    }
}
