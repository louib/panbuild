extern crate yaml_rust;
extern crate clap;

// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::{YamlLoader};
use clap::{Arg, App};
use std::fs;
use std::path;

mod manifest;

fn main() {
    let matches = App::new("panbuild")
                          .version("0.0.1")
                          .author("louib <code@louib.net>")
                          .about("The universal build manifest converter.")
                          .arg(Arg::with_name("input_file")
                               .short("i")
                               .long("input-file")
                               .value_name("MANIFEST")
                               .required(false)
                               .help("Path of the input build manifest.")
                               .takes_value(true))
                          .get_matches();

    let input_file = matches.value_of("input_file").unwrap();
    let input_file_path = path::Path::new(input_file);

    let manifest_content = fs::read_to_string(input_file_path).unwrap();
    let manifest = YamlLoader::load_from_str(&manifest_content);

    println!("Hello, world!");
}
