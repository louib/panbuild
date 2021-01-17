use std::path;
use std::env;
use std::process::exit;
use std::io::{self, BufRead};

fn main() {
    let mut exit_code = 0;

    // TODO might need to use std::env::args_os instead, if
    // the args contain unicode.
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Require 1 argument.");
        exit(1);
    }

    let command_name = &args[1];

    if command_name == &"import-modules".to_string() {
        let mut modules: Vec<panbuild::manifests::manifest::AbstractModule> = vec![];

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line_str = line.unwrap();
            let repo_dir = match panbuild::utils::clone_git_repo(line_str.to_string()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Could not clone repo {}", line_str);
                    continue;
                },
            };
            let repo_file_paths = match panbuild::utils::get_all_paths(path::Path::new(&repo_dir)) {
                Ok(paths) => paths,
                Err(message) => {
                    eprintln!("Could not get the file paths :sad: {}", message);
                    continue;
                }
            };
            for file_path in repo_file_paths.iter() {
                let abstract_manifest = match panbuild::manifests::manifest::AbstractManifest::load_from_file(file_path.to_str().unwrap().to_string()) {
                    Some(m) => m,
                    None => {
                        continue;
                    },
                };
                let manifest_modules = match abstract_manifest.get_modules() {
                    Ok(m) => m,
                    Err(m) => {
                        continue;
                    },
                };
                for module in manifest_modules {
                    modules.push(module);
                }
            }
        }
    }

    exit(exit_code);
}
