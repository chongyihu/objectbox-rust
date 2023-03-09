# Objectbox for rust
Unofficial rust support for ObjectBox database.

See the [example crate](example/src/main.rs).

## Example
```rust
extern crate objectbox;

use objectbox::{macros::entity, opt::Opt, store::Store};

mod objectbox_gen;
use objectbox_gen as ob;

#[derive(Debug)]
#[entity]
pub struct Entity {
    #[id]
    id: u64,
    hello: String,
}

fn main() {
    let mut model = ob::make_model();
    let opt = Opt::from_model(&mut model);
    let trait_map = ob::make_factory_map();
    let store = Store::new(opt, trait_map).expect("crash");

    let mut box1 = store.get_box::<Entity>().expect("crash");

    let mut e_before = Entity {
        id: 0,
        hello: "Hello world!".to_string(),
    };

    let new_id = match box1.put(&mut e_before).expect("crash");

    match box1.get(new_id) {
        Err(err) => panic!("{err}"),
        Ok(found_item) => {
            if let Some(object) = found_item {
                println!("{}", object.hello);
            }
        }
    }
}
```

## Abstract roadmap
* Fix Query bug: any condition returns all the objects from a box
* Remove stored error pattern: return the Error immediately, map the Ok,
  so they can be chained with ?-op / try
* Support fields with `Option<P>` where `P` is some primitive type
* Write more tests, especially for all condition ops
## Problems solved, 2023 Feb
* Code generation from struct entities with macros
* Code generation for injecting the model to Store
* Weave traits to make blankets, so objects can be created, flattened, inflated.

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer

## TODO (Nice to haves)
* Reimplement macros with [darling's](https://github.com/TedDriggs/darling/blob/master/examples/consume_fields.rs) [cleaner abstractions (example how)](https://github.com/Buggaboo/lean_buffer/blob/main/macros/src/lib.rs).
