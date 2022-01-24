use prost_serde::build_with_serde;
use std::process::Command;
use std::{fs, option_env};

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    let opts = build_with_serde(include_str!("build_opts.json"));
    let output = &opts
        .output
        .unwrap_or_else(|| panic!("Failed to build the protobuf files with build_opts.json."));
    let gen = &format!("{}/gen.rs", output);
    fs::rename(format!("{}/abi.rs", output), gen)
        .unwrap_or_else(|e| panic!("Failed to move proto files. Error: {:?}", e));
    Command::new("cargo")
        .args(&["fmt", "--", gen.as_str()])
        .status()
        .expect("cargo fmt failed");
}
