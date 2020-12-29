use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn get_file_paths(dir: String) -> Vec<String> {
    return vec![];
}

// one possible implementation of walking a directory only visiting files
// Taken from https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
