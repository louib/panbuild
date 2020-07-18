use std::process::Command;

// FIXME make this relative to the current script.
static FIXTURES_DIR: &str = "tests/fixtures";
static OUTPUT_DIR: &str = "tests/output";
static EXPECTED_DIR: &str = "tests/expected";

fn cleanup_output_dir() {
    Command::new("mkdir")
        .arg(format!("-p {}", OUTPUT_DIR))
        .output()
        .expect("");
    // rm "{0}/*"
}

fn main() {
    Command::new("command")
        .arg("-v panbuild")
        .output()
        .expect("");

    println!("🔍 Starting functional test suite for panbuild. 🔍");
}
