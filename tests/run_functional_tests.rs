use std::process::Command;

// FIXME make this relative to the current script.
static FIXTURES_DIR: String = "tests/fixtures";
static OUTPUT_DIR: String = "tests/output";
static EXPECTED_DIR: String = "tests/expected";

fn cleanup_output_dir() {
    // mkdir -p "$OUTPUT_DIR"
    // rm "{0}/*"
}

fn main() {
    Command::new("command")
        .args("-v panbuild")
        .output()
        .expect("");

    println!("🔍 Starting functional test suite for panbuild.");
}
