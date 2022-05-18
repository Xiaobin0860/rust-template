use prost_build_config::{BuildConfig, Builder};

fn main() {
    let config: BuildConfig = serde_yaml::from_str(include_str!("build_opts.yml")).unwrap();
    Builder::from(config).build_protos();

    std::fs::rename("src/pb/abi.rs", "src/pb/gen.rs")
        .unwrap_or_else(|e| panic!("Failed to move proto files. Error: {:?}", e));

    std::process::Command::new("cargo")
        .args(&["fmt", "--", "src/pb/gen.rs"])
        .status()
        .expect("cargo fmt failed");

    println!("cargo:rerun-if-changed=../protobuf");
    println!("cargo:rerun-if-changed=build.rs");
}
