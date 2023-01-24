# Objectbox for rust

See the example crate, WiP.

## Abstract roadmap
* Flesh out
  * Drop trait for ob
  * Store
  * Box
  * Query
* Write tests along the way
* Decide what to do with:
  * transient struct properties aka unmapped properties, e.g. `Box<dyn trait>`, Box<SomeType>, etc. (initial idea: only allow `Option<P>` where P primitive?)
  * also generically typed properties (e.g. panic when parsed generic param)
* Consider rusty [planus](https://github.com/planus-org/planus) instead of [the OG flatbuffers lib](https://github.com/google/flatbuffers/tree/master/rust/flatbuffers)
* Experiment with memory pools, especially on the fb side of things (what to do with unbounded Vector types?)
## Problems solved, 2023 Jan
* Code generation from struct entities with macros
* Code generation for injecting the model to Store

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer