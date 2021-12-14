use std::process::Command;

fn main() {
    Command::new("cargo")
        .args(["install", "perseus-cli", "--version 0.3.0-beta.22"])
        .output()
        .expect("failed to install perseus-cli");
}
