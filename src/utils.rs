use std::fs::{self, DirEntry};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::SystemTime;

pub fn clone_git_repo(repo_url: String) -> Result<String, String> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let project_name = repo_url.split("/").last().unwrap();
    let repo_dir = format!("/tmp/panbuild-git-clone-{}-{}", project_name, timestamp.unwrap().as_secs());
    if let Err(e) = fs::create_dir(&repo_dir) {
        return Err(e.to_string());
    }

    println!("Cloning repo {}", repo_url);
    let mut output = Command::new("git")
        .arg("clone")
        .arg(repo_url.to_string())
        .arg(&repo_dir)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut output = match output.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    if !output.status.success() {
        return Err("Could not clone repo.".to_string());
    }

    Ok(repo_dir)
}

pub fn get_git_repo_initial_commits(repo_path: String) -> Result<Vec<String>, String> {
    // FIXME there can actually be more than 1 parentless commit
    // in a git repo, in the case of a merger. A parentless commit
    // can also be found in multiple projects in the case of a fork.
    println!("Getting initial commit for repo at {}", repo_path);

    let mut output = Command::new("git")
        .arg("rev-list")
        .arg("max-parents=0".to_owned())
        .arg("HEAD")
        .arg("work-tree=".to_owned() + &repo_path.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut output = match output.wait_with_output() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    if !output.status.success() {
        return Err("Could not clone repo.".to_string());
    }
    let all_hashes = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    Ok(all_hashes.split('\n').map(|s| s.to_string()).collect())
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
