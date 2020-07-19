pub mod snap;
pub mod npm;
pub mod debian;
pub mod flatpak;
pub mod pyproject;
pub mod manifest;

pub fn parse() {
    println!("called `manifests::parse()`");
}

pub fn dump() {
    println!("called `manifests::dump()`");
}

pub fn get_type(filename: &str, file_content: &str) {
    println!("called `manifests::match_filename()`");
}
