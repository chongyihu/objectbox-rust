
extern crate objectbox;
extern crate flatbuffers;

use objectbox::macros::entity;

// uncomment the next three lines
// when the mod hasn't been generated yet
mod objectbox_gen;
use objectbox_gen as ob;
use objectbox::traits::FBOBBridge;

mod table_generated;

use flatbuffers::FlatBufferBuilder;
use crate::table_generated::ob::finish_entity_buffer;
use crate::table_generated::ob::entityArgs;
use crate::table_generated::ob::entity;

/*
table entity {
    id:  ulong;

	t_i8:   byte;
	t_u8:   ubyte;
	t_bool: bool;

    t_i16:  short;
    t_u16:  ushort;

    t_i32:  int;
	t_u32:  uint;
    t_f32:  float;

	t_u64:  ulong;
    t_i64:  long;
    t_f64:  double;

    t_string: string;

    t_vec_u8: [ubyte];
    t_vec_string: [string];
}
*/

#[derive(Debug)]
#[entity]
pub struct Entity {
  #[id]
  id: u64,
  t_i8:   i8,
  t_u8:   u8,
  t_bool: bool,

  t_i16:  i16,
  t_u16:  u16,

  t_i32:  i32,
  t_u32:  u32,
  t_f32:  f32,

  t_u64:  u64,
  t_i64:  i64,
  t_f64:  f64,

  t_string: String,

  t_vec_u8: Vec<u8>,
  t_vec_string: Vec<String>,
}

fn fb_make_entity(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>) {
    dest.clear();
    builder.reset();
    let args = entityArgs {
        id: 1,
        t_i8: 1,
        t_u8: 2,
        t_bool: false,
        t_i16: 3,
        t_u16: 4,
        t_i32: 5,
        t_u32: 6,
        t_f32: 7.0,
        t_u64: 8,
        t_i64: 9,
        t_f64: 10.0,
        t_string: None,
        t_vec_u8: None,
        t_vec_string: None,
    };

    let entity_offset = entity::create(builder, &args);
    finish_entity_buffer(builder, entity_offset);
    dest.extend_from_slice(builder.finished_data());
}

fn ob_make_entity(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>) {
    dest.clear();
    builder.reset();
    let e1 = Entity {
        id: 1,
        t_i8: 1,
        t_u8: 2,
        t_bool: false,
        t_i16: 3,
        t_u16: 4,
        t_i32: 5,
        t_u32: 6,
        t_f32: 7.0,
        t_u64: 8,
        t_i64: 9,
        t_f64: 10.0,
        t_string: "".to_string(),
        t_vec_u8: Vec::new(),
        t_vec_string: Vec::new(),
    };

    e1.to_fb(builder);
    dest.extend_from_slice(builder.finished_data());
}

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::FlatBufferBuilder;
    
    // use flatbuffers::{FlatBufferBuilder, Table};

    // use std::rc;
    // use objectbox::traits::{self, IdExt, FBOBBridge};

    #[test]
    fn compare_ob_vs_fb_created_entities() {
        let mut fbb = FlatBufferBuilder::new();
        let mut fb_out = Vec::<u8>::new();
        let mut ob_out = Vec::<u8>::new();
        fb_make_entity(&mut fbb, &mut fb_out); // contains specific values
        ob_make_entity(&mut fbb, &mut ob_out); // should more or less contain the same values

        assert_eq!(fb_out, ob_out);

        // unsafe {
        //     let trait_map = ob::make_factory_map();
        //     let f1 = trait_map.get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity>>>().unwrap().clone();
        //     let mut table = Table::new(vec.as_slice(), 4);
        //     let e1_copy = f1.make(&mut table);

        //     assert_eq!(e1_copy.id, e1.id);
        // }

    }
}
