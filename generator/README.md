# Generator

This crate along with the macros crate are required to generate the `src/objectbox-model.json` and `src/objectbox.rs`
for your project's crate.

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
  let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  gen::generate_assets(&out_dir, &cargo_manifest_dir);
}
```

Please check the example crate for more details.

## TODO
* Check rust char conversion back and fro is correct in relation with OB
* Support `Option<primitive>` types, e.g. Option<u32>, Option<String> etc.
* Implement relations, and everything else, roadmap?
## Other interesting avenues of research
* leverage macro (see macros package)
  * -> generate .fbs (maybe requires extending genco), simple use-case with fb tables
  * -> generate .rs accessors with the [flatc-rust](https://github.com/frol/flatc-rust)