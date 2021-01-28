use std::path;
use std::fs;
use std::env;
use std::process::exit;
use std::io::{self, BufRead, Write};

use panbuild::manifests::manifest::AbstractManifest;
use panbuild::modules::SoftwareModule;

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
        let mut modules: Vec<SoftwareModule> = vec![];

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line_str = line.unwrap();
            let repo_dir = match panbuild::utils::clone_git_repo(line_str.to_string()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Could not clone repo {}: {}", line_str, e);
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
                let abstract_manifest = match AbstractManifest::load_from_file(file_path.to_str().unwrap().to_string()) {
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

            let modules_dump = serde_yaml::to_string(&modules).unwrap();
            let output_file_path = "./modules.yaml".to_string();
            match fs::write(path::Path::new(&output_file_path), modules_dump) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("could not write file {}.", &output_file_path);
                    return;
                }
            };
        }
    }

    if command_name == &"import-flatpak-shared-modules".to_string() {
        let mut modules: Vec<SoftwareModule> = vec![];
        let mut db = panbuild::db::Database::get_database();
        let repo_path = match panbuild::utils::clone_git_repo(
            "https://github.com/flathub/shared-modules.git".to_string()
        ) {
            Ok(p) => p,
            Err(e) => {
                panic!("Could not glone flathub shared modules repo.");
            }
        };
        let all_repo_paths = match panbuild::utils::get_all_paths(path::Path::new(&repo_path)) {
            Ok(p) => p,
            Err(e) => {
                panic!("Could not get paths in flathub shared modules repo.");
            }
        };
        for file_path in &all_repo_paths {
            let file_path_str = file_path.to_str().unwrap();
            let file_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("could not read file {}: {}.", file_path_str, e);
                    continue;
                }
            };

            let module: panbuild::modules::SoftwareModule = match serde_json::from_str(&file_content) {
                Ok(m) => m,
                Err(e) => continue,
            };

            db.add_module(module);
            print!("Parsed Flatpak module at {}.", file_path_str);
        }
    }

    exit(exit_code);
}
