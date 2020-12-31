use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
// Taken from https://doc.rust-lang.org/std/fs/fn.read_dir.html
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
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

pub fn get_all_paths(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let entries = match fs::read_dir(dir) {
        Ok(paths) => paths,
        Err(err) => return Err(format!("Error reading directory {}: {}", dir.to_str().unwrap(), err)),
    };
    let entries = entries.map(|res| res.map(|e| e.path()));
    let entries = match entries.collect::<Result<Vec<_>, io::Error>>() {
        Ok(entry) => entry,
        Err(err) => return Err(format!("Error collecting the entries: {}", err)),
    };
    Ok(entries)
}
