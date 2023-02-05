use objectbox::c;
use objectbox::entity_builder;
use objectbox::flatbuffers;
use objectbox::map;
use objectbox::model;
use objectbox::traits;
use std::rc;
impl traits::IdExt for crate::Entity {
    fn get_id(&self) -> c::obx_id {
        self.id
    }
    fn set_id(&mut self, id: c::obx_id) {
        self.id = id;
    }
}
impl traits::FBOBBridge for crate::Entity {
    fn to_fb(&self, builder: &mut flatbuffers::FlatBufferBuilder) {
        builder.reset();
        let str_28 = builder.create_string(self.t_string.as_str());
        let byte_vec_30 = builder.create_vector(&self.t_vec_u8.as_slice());
        let strs_vec_32 = self
            .t_vec_string
            .iter()
            .map(|s| builder.create_string(s.as_str()))
            .collect::<Vec<flatbuffers::WIPOffset<&str>>>();
        let vec_32 = builder.create_vector(strs_vec_32.as_slice());
        let wip_offset_unfinished = builder.start_table();
        builder.push_slot::<u64>(4, self.id, 0);
        builder.push_slot::<i8>(6, self.t_i8, 0);
        builder.push_slot::<u8>(8, self.t_u8, 0);
        builder.push_slot::<bool>(10, self.t_bool, false);
        builder.push_slot::<i16>(12, self.t_i16, 0);
        builder.push_slot::<u16>(14, self.t_u16, 0);
        builder.push_slot::<i32>(16, self.t_i32, 0);
        builder.push_slot::<u32>(18, self.t_u32, 0);
        builder.push_slot::<f32>(20, self.t_f32, 0.0);
        builder.push_slot::<u64>(22, self.t_u64, 0);
        builder.push_slot::<i64>(24, self.t_i64, 0);
        builder.push_slot::<f64>(26, self.t_f64, 0.0);
        builder.push_slot_always(28, str_28);
        builder.push_slot_always(30, byte_vec_30);
        builder.push_slot_always(32, vec_32);
        let wip_offset_finished = builder.end_table(wip_offset_unfinished);
        builder.finish_minimal(wip_offset_finished);
    }
}
impl traits::FactoryHelper<crate::Entity> for traits::Factory<crate::Entity> {
    fn make(&self, table: &mut flatbuffers::Table) -> crate::Entity {
        let mut object = self.new_entity();
        let crate::Entity {
            id,
            t_i8,
            t_u8,
            t_bool,
            t_i16,
            t_u16,
            t_i32,
            t_u32,
            t_f32,
            t_u64,
            t_i64,
            t_f64,
            t_string,
            t_vec_u8,
            t_vec_string,
        } = &mut object;
        unsafe {
            *id = table.get::<u64>(4, Some(0)).unwrap();
            *t_i8 = table.get::<i8>(6, Some(0)).unwrap();
            *t_u8 = table.get::<u8>(8, Some(0)).unwrap();
            *t_bool = table.get::<bool>(10, Some(false)).unwrap();
            *t_i16 = table.get::<i16>(12, Some(0)).unwrap();
            *t_u16 = table.get::<u16>(14, Some(0)).unwrap();
            *t_i32 = table.get::<i32>(16, Some(0)).unwrap();
            *t_u32 = table.get::<u32>(18, Some(0)).unwrap();
            *t_f32 = table.get::<f32>(20, Some(0.0)).unwrap();
            *t_u64 = table.get::<u64>(22, Some(0)).unwrap();
            *t_i64 = table.get::<i64>(24, Some(0)).unwrap();
            *t_f64 = table.get::<f64>(26, Some(0.0)).unwrap();
            if let Some(s) = table.get::<flatbuffers::ForwardsUOffset<&str>>(28, None) {
                *t_string = s.to_string();
            }
            let fb_vec_t_vec_u8 = table
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<u8>>>(30, None);
            if let Some(bv) = fb_vec_t_vec_u8 {
                *t_vec_u8 = bv.bytes().to_vec();
            }
            let fb_vec_t_vec_string = table
                .get::<
                    flatbuffers::ForwardsUOffset<
                        flatbuffers::Vector<flatbuffers::ForwardsUOffset<&str>>,
                    >,
                >(32, None);
            if let Some(sv) = fb_vec_t_vec_string {
                *t_vec_string = sv.iter().map(|s| s.to_string()).collect();
            }
        }
        object
    }
    fn get_entity_id(&self) -> c::obx_schema_id {
        self.schema_id
    }
    fn new_entity(&self) -> crate::Entity {
        crate::Entity {
            id: 0,
            t_i8: 0,
            t_u8: 0,
            t_bool: false,
            t_i16: 0,
            t_u16: 0,
            t_i32: 0,
            t_u32: 0,
            t_f32: 0.0,
            t_u64: 0,
            t_i64: 0,
            t_f64: 0.0,
            t_string: String::from(""),
            t_vec_u8: Vec::<u8>::new(),
            t_vec_string: Vec::<String>::new(),
        }
    }
}
pub fn make_model() -> model::Model {
    let builder = Box::new(entity_builder::EntityBuilder::new());
    model::Model::new(builder)
        .entity("Entity", 1, 15452510860359729572)
        .property("id", 1, 10313449018668933947, 6, 129)
        .property("t_i8", 2, 3830372827350786645, 2, 0)
        .property("t_u8", 3, 10952233183169727153, 2, 8192)
        .property("t_bool", 4, 15571530370830065955, 1, 0)
        .property("t_i16", 5, 11613688133037110828, 3, 0)
        .property("t_u16", 6, 18265431932756421895, 3, 8192)
        .property("t_i32", 7, 2725745992398581889, 5, 0)
        .property("t_u32", 8, 11742068401161511297, 5, 8192)
        .property("t_f32", 9, 902343158669776659, 7, 0)
        .property("t_u64", 10, 12455433643327311951, 6, 8192)
        .property("t_i64", 11, 11085045505002422551, 6, 0)
        .property("t_f64", 12, 8620002425694515782, 8, 0)
        .property("t_string", 13, 1926874115772933628, 9, 0)
        .property("t_vec_u8", 14, 7803916618768440517, 23, 0)
        .property("t_vec_string", 15, 16560557739183397927, 30, 0)
        .last_property_id(15, 16560557739183397927)
        .last_entity_id(1, 15452510860359729572)
}
pub fn make_factory_map() -> map::AnyMap {
    let mut map = map::AnyMap::new();
    let f1 = rc::Rc::new(traits::Factory::<crate::Entity> {
        _required_for_generic_trait: None,
        schema_id: 1,
    }) as rc::Rc<dyn traits::FactoryHelper<crate::Entity>>;
    map.insert(f1);
    map
}
