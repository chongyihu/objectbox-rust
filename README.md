# Objectbox for rust

See the example crate, WiP.

## Abstract roadmap
* Flesh out Store, Box, Query, to actually use the underlying objectbox-c database.

## Problems solved, 2023 Jan
* Code generation from struct entities with macros
* Code generation for injecting the model to Store

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer