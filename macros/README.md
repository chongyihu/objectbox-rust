# Macros

This project contains the macros required to generate
the `objectbox-model.json` file, from rust structs.

For example:

```rust
extern crate objectbox;

use objectbox::macros::{entity, index};

#[entity]
struct Entity {
  #[index]
  index: u64,
  t_bool : bool,
  t_u8 : u8,
  t_i8 : i8,
  t_i16: i16,
  t_u16: u16,
  t_char: char,
  t_i32: i32,
  t_u32: u32,
  t_u64: u64,
  t_i64: i64,
  t_f32: f32,
  t_f64: f64,
  t_string: String,
}
```

## TODO
* Support `Option<primitive>` types, e.g. Option<u32>, Option<String> etc.
  afaik OB supports nullable fields