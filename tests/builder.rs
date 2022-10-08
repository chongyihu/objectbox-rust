use objectbox;
extern crate builder;

use builder::{entity, index, transient, unique};

// #[entity]
// struct SomeEmptyType; // expected: ignore completely

#[entity]
struct CorrectType {
  t_bool : bool,
  #[unique(uid=1339)]
  t_u32 : u32,
}

#[entity(uid=1337)]
struct LessEmptyTypeWithParams {
  #[index(uid=1338)]
  t_bool : bool,
  #[transient]
  t_u32 : u32,
  t_u64 : u64,
  t_double : f64,
}

// expected: panic
// #[entity]
// enum Panic1 {
//   stuff, more_stuff
// }

// expected: panic
// #[entity(p=1337)]
// struct Panic2 {
//   t_u32 : u32,
//   t_u64 : u64,
//   t_double : f64,
// }

#[test]
fn test_entity_codegen() {
  let an_entity = CorrectType { t_bool : true, t_u32 : 1337 };
  assert_eq!(an_entity.t_u32, 1337);
}
