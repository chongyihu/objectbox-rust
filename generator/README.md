# Generator

This crate along with the macros crate are required to generate the `src/objectbox-model.json` and `src/objectbox_gen.rs`
for your project's crate.

This crate is responsible for generating the `src/objectbox_gen.rs`.

Applied on `cargo.toml`:

```toml
[package]
name = "my objectbox project"
version = "0.1.0"
edition = "2021"

[dependencies]
cargo-make = "0.36.3"
#objectbox = { path = "./objectbox" }
objectbox = "0.1.0"

[build-dependencies]
#objectbox = { path = "./objectbox" }
objectbox = "0.1.0" # whichever is appropriate
glob = "0.3"
```

Applied on your crate's `build.rs` to kickstart code generation:

```rust
use objectbox::generator as gen;
use std::path::PathBuf;
use std::env;

fn main() {
  let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  gen::generate_assets(&out_dir, &target_dir);
}
```

Please check the example crate for more details.

## TODO
* Implement relations, and everything else, roadmap?
## Other interesting avenues of research
* leverage macro (see macros package)
  * spawn new project to generate .fbs (maybe requires extending genco), simple use-case with fb tables
  * spawn new project to generate .rs accessors with the [flatc-rust](https://github.com/frol/flatc-rust)