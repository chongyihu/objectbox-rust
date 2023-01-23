# Objectbox for rust

See the example crate, WiP.

## Abstract roadmap
* Flesh out Store, Box, Query, to actually use the underlying objectbox-c database.
* Write tests along the way
* Transient struct properties aka unmapped properties, it's not going to be feasible to generate whatever is required for the ad hoc value going into the struct. Transients could be of the Option type though, then we would only need to generate `None`.
## Problems solved, 2023 Jan
* Code generation from struct entities with macros
* Code generation for injecting the model to Store

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer