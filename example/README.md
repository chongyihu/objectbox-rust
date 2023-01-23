# HOWTO

Generating `objectbox_gen.rs` and `objectbox-model.json`
require running `cargo build` at least twice.

1. Once to generate `Entity`.objectbox.info for each entity. This is carried out by `macros` crate.

2. Once more to generate the files mentioned in the first sentence.
  this is carried out by `generator` crate. Triggered by the `build.rs` file.