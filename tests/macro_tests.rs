extern crate chrono;
extern crate objectbox_macros;

use chrono::{DateTime, Utc};
use objectbox_macros::{entity, index, sync, transient, unique};

/*
#[entity]
struct SomeEmptyType; // expected: panic
*/

#[sync]
#[entity(id = 1, uid = 1337)]
struct TypeTest {
    t_bool: bool,
    #[unique(id = 1, uid = 1339)]
    t_u32: u32,
}

#[entity(uid = 1338, id = 2)]
struct TypeTestAgain {
    #[index(uid = 1338, id = 2)]
    t_bool: bool,
    t_u64: u64,
    #[property(id = 1333, uid = 111111)]
    t_double: f64,
    // ha, the attribute survived in spite of being a reserved keyword
    #[unique(type = 12)]
    t_i64: i64,
    // #[index]
    // t_datetime : DateTime<Utc>,
    // #[transient]
    // t_ignored : u32,
}

#[entity]
struct MoreTypeTests {
    t_bool: bool,
    t_u8: u8,
    t_i8: i8,
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

// expected: panic
// #[entity]
// enum Panic1 {
//   stuff, more_stuff
// }

// expected: ignored
#[entity(bad_parameter_ignored = 1337)]
struct Panic2 {
    t_u32: u32,
    t_u64: u64,
    t_double: f64,
}

#[test]
fn test_entity_codegen() {
    let an_entity = TypeTest {
        t_bool: true,
        t_u32: 1337,
    };
    assert_eq!(an_entity.t_u32, 1337);
}
