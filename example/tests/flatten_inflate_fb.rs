use std::rc;

use example::make_factory_map;
use objectbox::flatbuffers::{FlatBufferBuilder, Table};
use objectbox::traits;

use objectbox::traits::FBOBBridge;

#[test]
fn test_write_and_read_fb() {
    let trait_map2 = make_factory_map();
    let f1 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<example::Entity>>>()
        .unwrap()
        .clone();
    let f2 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<example::Entity2>>>()
        .unwrap()
        .clone();
    let f3 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<example::Entity3>>>()
        .unwrap()
        .clone();

    let mut e1 = f1.new_entity();
    let mut e2 = f2.new_entity();
    let mut e3 = f3.new_entity();

    e1.id = 0xFFFFFFFF;
    e2.id = 0xFFFFFFFF;
    e3.id = 0xFFFFFFFF;

    let mut fbb = FlatBufferBuilder::new();

    unsafe {
        e3.flatten(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();

        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e3_copy = f3.make(&mut table);

        assert_eq!(e3_copy.id, e3.id);
    }

    unsafe {
        e2.flatten(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();

        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e2_copy = f2.make(&mut table);

        assert_eq!(e2_copy.id, e2.id);
    }

    unsafe {
        e1.flatten(&mut fbb);
        let vec = Vec::from(fbb.finished_data());
        let vec_slice = vec.as_slice();

        let mut table = Table::new(vec_slice, vec_slice[0].into());
        let e1_copy = f1.make(&mut table);

        assert_eq!(e1_copy.id, e1.id);
    }
}
