mod table_generated;

use flatbuffers::FlatBufferBuilder;
use crate::table_generated::ob::finish_entity_buffer;
use crate::table_generated::ob::entityArgs;
use crate::table_generated::ob::entity;

fn make_entity(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>) {
    dest.clear();
    builder.reset();
    let args = entityArgs {
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

    let finished_data = builder.finished_data();
    dest.extend_from_slice(finished_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity() {
        // TODO
        assert_eq!(4, 4);
    }
}
