# Objectbox for rust
Unofficial rust support for ObjectBox database.

See the example crate, WiP.

## Abstract roadmap
* Fix Query bug: any condition returns all the objects from a box
* Write more tests
## Problems solved, 2023 Feb
* Code generation from struct entities with macros
* Code generation for injecting the model to Store
* Weave traits to make blankets, so objects can be created, flattened, inflated.

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer

## TODO (Nice to haves)
* Support fields with `Option<P>` where `P` is some primitive type
* Reimplement macros with [darling's](https://github.com/TedDriggs/darling/blob/master/examples/consume_fields.rs) [cleaner abstractions (example how)](https://github.com/Buggaboo/lean_buffer/blob/main/macros/src/lib.rs).
