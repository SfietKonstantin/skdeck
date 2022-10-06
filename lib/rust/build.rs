use cbindgen::Config;
use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config_file = Path::new(&crate_dir).join("cbindgen.toml");
    let config = Config::from_file(config_file).expect("Failed to read cbindgen.toml");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
}
