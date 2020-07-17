extern crate yaml_rust;

// use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::{YamlLoader};

fn main() {
    let s = "---";
    YamlLoader::load_from_str(&s).unwrap();

    println!("Hello, world!");
}
