use objectbox::generator as gen;
use std::path::PathBuf;
use std::env;

fn main() {
  let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  gen::generate_assets(&out_dir, &cargo_manifest_dir);
}
