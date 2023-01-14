use std::collections::HashMap;
use std::any::{Any, TypeId};

struct ModelDefinition {
  model_info: ModelInfo,
  bindings: HashMap, // <std::any::TypeId, blanket trait>
}

/// _Not_ feasible initial idea: this is not known ahead of time by store and box
/// because it needs to be generated at the same crate and module,
/// before store and box are also compiled.
/// Also it's another abstraction layer.
// TODO reformat the following code block properly
/// impl<T> Entity {
///   fn to_FB(self, builder: &fb.Builder);
///   fn from_FB(store: &mut Store, byte_buffer: &ByteBuffer) -> T;
///   fn get_id(&self) -> u64;
///   fn set_id(&mut self, id: u64);
///   fn get_type(&self) -> std::any::TypeId;
///   fn to_one_relations(&self) -> ...
///   fn to_many_relations(&self) -> ...
/// }

// TODO
/// My gut feeling says use extension trait on the Entity directly
/// since the closure signatures all suggest that,
/// except objectFromOB from the dart impl, which could be an Entity trait factory
/// with signature: Entity::fromFB(store, fbData).
/// During compile time, the store or box only is concerned about
/// that the traits are implemented on the object being passed
/// in compile time, not runtime (unlike dart's impl)
/// In this case, we eliminate the need for ModelDefinition and EntityDefiniton
/// All we need are cross-concern cutting traits, and pass those instances around as mut refs.
