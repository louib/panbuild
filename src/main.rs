extern crate yaml_rust;
extern crate clap;

// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::{YamlLoader};
use clap::{Arg, App, ArgMatches};
use std::fs;
use std::path;
use std::process::{exit};

mod manifest;
mod snap;
mod npm;
mod flatpak;
mod pyproject;

fn main() {
    let matches: ArgMatches = App::new("panbuild")
                          .version("0.0.1")
                          .author("louib <code@louib.net>")
                          .about("The universal build manifest converter.")
                          .arg(Arg::with_name("input_file")
                               .short("i")
                               .long("input-file")
                               .takes_value(true)
                               .value_name("MANIFEST")
                               .required(false)
                               .help("Path of the input build manifest."))
                          .arg(Arg::with_name("version")
                               .short("V")
                               .long("version")
                               .required(false)
                               .help("Show the version and exit."))
                          .get_matches();

    if matches.is_present("version") {
        println!("0.0.1");
        exit(0);
    }

    let input_file = matches.value_of("input_file").unwrap();
    let input_file_path = path::Path::new(input_file);

    let manifest_content = fs::read_to_string(input_file_path).unwrap();
    let manifest = YamlLoader::load_from_str(&manifest_content);

    println!("Hello, world!");
    exit(0);

}
