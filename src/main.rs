extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    let source_manifest = YamlLoader::load_from_str(s).unwrap();

    println!("Hello, world!");
}
