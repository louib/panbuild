use std::process::{exit};

fn die(message: &str) {
    println!("💣 {} 💣", message);
    // TODO accept exit code as optional parameter.
    exit(1);
}
