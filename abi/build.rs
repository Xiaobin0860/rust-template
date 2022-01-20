use prost_serde::build_with_serde;
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
    let ref output = opts
        .output
        .unwrap_or_else(|| panic!("Failed to build the protobuf files with build_opts.json."));
    fs::rename(format!("{}/abi.rs", output), format!("{}/gen.rs", output))
        .unwrap_or_else(|e| panic!("Failed to move proto files. Error: {:?}", e));
}
