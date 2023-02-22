# Objectbox for rust

See the example crate, WiP.

## Abstract roadmap
* Flesh out:
  * Query
* Write tests along the way
## Problems solved, 2023 Feb
* Code generation from struct entities with macros
* Code generation for injecting the model to Store

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer
* Use std traits, don't reinvent the wheel, unless necessary

## Interesting research avenues
* Decide what to do with:
  * transient struct properties aka unmapped properties, e.g. `Box<dyn trait>`, `Box<SomeType>`, etc. (initial idea: only allow `Option<P>` where `P` primitive?)
  * also generically typed properties (e.g. panic when parsed generic param)
* Experiment with memory pools, especially on the fb side of things (solve: what to do with unbounded aggregate types, e.g. Vector, Map, Set? Separate table?)
* Run [Miri](https://github.com/rust-lang/miri) to scan for UBs across ARCHs
