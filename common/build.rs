fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/proto")
        .compile_protos(&["abi.proto"], &[".", "../proto/", "../proto/"])
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../proto/abi.proto");
}