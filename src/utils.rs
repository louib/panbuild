use std::fs::{self, DirEntry};
use std::io::{stdin, stdout, Write};
use std::process::{Command, Output, Stdio};
use std::path::Path;
use std::time::SystemTime;

pub fn clone_git_repo(repo_url: String) -> Result<String, String> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let repo_dir = format!("/tmp/panbuild-git-clone-{}", timestamp.unwrap().as_secs());
    if let Err(e) = fs::create_dir(&repo_dir) {
        return Err(e.to_string());
    }

    println!("Cloning repo {}", repo_url);
    let mut output = Command::new("git")
        .arg("clone")
        .arg(repo_url.to_string())
        .arg(&repo_dir)
        .spawn();

    let mut output = match output {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };

    Ok(repo_dir)
}

pub fn get_all_paths(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut all_paths: Vec<std::path::PathBuf> = vec![];

    let dir_entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => return Ok(vec![]),
    };
    for entry in dir_entries {
        let entry_path = entry.unwrap().path();
        if entry_path.is_dir() {
            let mut dir_paths: Vec<std::path::PathBuf> = get_all_paths(&entry_path)?;
            all_paths.append(&mut dir_paths);
        } else {
            all_paths.push(entry_path);
        }
    }

    Ok(all_paths)
}

pub fn ask_yes_no_question(question: String) -> bool {
    let mut answer = String::new();
    print!("{}? [Y/n]: ", question);
    let _ = stdout().flush();
    stdin().read_line(&mut answer).expect("Error while reading answer for question.");
    if let Some('\n') = answer.chars().next_back() {
        answer.pop();
    }
    if let Some('\r') = answer.chars().next_back() {
        answer.pop();
    }
    if answer == "Y" || answer == "y" {
        return true;
    }
    return false;
}
