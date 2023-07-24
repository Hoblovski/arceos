use std::fs;
use std::path::Path;

fn main() {
    let dest_path = Path::new("src/").join("seed.rs");
    let init_seed = rand::random::<usize>();
    let contents = format!(
        "
//! GENERATED FILE. DO NOT EDIT.
pub const INIT_SEED: usize = {};
        ",
        init_seed
    );
    fs::write(&dest_path, contents).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}