use std::env;

fn main() {
    #[cfg(unix)]
    create_cbindgen_header();
}

fn create_cbindgen_header() {
    let config = cbindgen::Config::from_file("cbindgen.toml").unwrap();
    cbindgen::Builder::new()
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../examples/c_demo/ffi_client.h");
}
