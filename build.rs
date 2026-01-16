use std::process::Command;

fn main() {
    // Get current UTC time in a readable format
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%d %H:%M:%S UTC"])
        .output()
        .expect("Failed to execute date command");

    let build_time = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rerun-if-changed=build.rs");
}
