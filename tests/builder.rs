use objectbox;
extern crate builder;

use builder::{entity, index, transient, unique, sync};

// #[entity]
// struct SomeEmptyType; // expected: ignore completely

#[sync]
#[entity(id=1, uid=3)]
struct Type_Test {
  t_bool : bool,
  #[unique(id=1,uid=1339)]
  t_u32 : u32,
}

#[entity(uid=1337)]
struct Type_Test_Again {
  #[index(uid=1338)]
  t_bool : bool,
  #[transient]
  t_u32 : u32,
  t_u64 : u64,
  #[property(id=1333, uid=111111)]
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
  let an_entity = Type_Test { t_bool : true, t_u32 : 1337 };
  assert_eq!(an_entity.t_u32, 1337);
}
