use std::process::Command;

fn main() {
    let lint_status = Command::new("npm")
        .args(["run", "css:lint"])
        .status()
        .unwrap_or_else(|error| {
            panic!("Failed to run css:lint: {}", error);
        });

    if !lint_status.success() {
        panic!("css:lint failed. Fix CSS issues before building.");
    }

    // Get current UTC time in a readable format
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%d %H:%M:%S UTC"])
        .output()
        .expect("Failed to execute date command");

    let build_time = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/styles.css");
    println!("cargo:rerun-if-changed=styles.css");
    println!("cargo:rerun-if-changed=package.json");
}
