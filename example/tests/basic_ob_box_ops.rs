use example::{make_factory_map, make_model, Entity, Entity2, Entity3};
use objectbox::traits::{self, IdExt};
use objectbox::{opt::Opt, store::Store};
use std::rc;

use serial_test::serial;

#[test]
#[serial]
fn test_box_put_and_count_and_remove_all() {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model).expect("crash");
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map).expect("crash");

    let mut box3 = store.get_box::<Entity3>().expect("crash");
    box3.remove_all().expect("crash");
    let mut box2 = store.get_box::<Entity2>().expect("crash");
    box2.remove_all().expect("crash");
    let mut box1 = store.get_box::<Entity>().expect("crash");
    box1.remove_all().expect("crash");

    let trait_map2 = make_factory_map();
    let f1 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<Entity>>>()
        .unwrap()
        .clone();
    let f2 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<Entity2>>>()
        .unwrap()
        .clone();
    let f3 = trait_map2
        .get::<rc::Rc<dyn traits::EntityFactoryExt<Entity3>>>()
        .unwrap()
        .clone();

    let mut e1 = f1.new_entity();
    let mut e2 = f2.new_entity();
    let mut e3 = f3.new_entity();

    if let Err(err) = box1.put(&mut e1) {
        panic!("{err}");
    }
    if let Err(err) = box2.put(&mut e2) {
        panic!("{err}");
    }
    if let Err(err) = box3.put(&mut e3) {
        panic!("{err}");
    }

    assert_eq!(false, e1.get_id() == 0, "Set new ID after put");
    assert_eq!(false, e2.get_id() == 0);
    assert_eq!(false, e3.get_id() == 0);

    assert_eq!(false, box1.is_empty().expect("crash"));
    assert_eq!(false, box2.is_empty().expect("crash"));
    assert_eq!(false, box3.is_empty().expect("crash"));

    assert_eq!(1, box1.count().expect("crash"));
    assert_eq!(1, box2.count().expect("crash"));
    assert_eq!(1, box3.count().expect("crash"));
    assert_eq!(1, box1.count_with_limit(1).expect("crash"));
    assert_eq!(1, box2.count_with_limit(1).expect("crash"));
    assert_eq!(1, box3.count_with_limit(1).expect("crash"));
    assert_eq!(1, box1.count_with_cursor().expect("crash"));
    assert_eq!(1, box2.count_with_cursor().expect("crash"));
    assert_eq!(1, box3.count_with_cursor().expect("crash"));

    box1.remove_all().expect("crash");
    assert!(box1.is_empty().expect("crash"));
    assert_eq!(0, box1.count_with_cursor().expect("crash"));

    box2.remove_all().expect("crash");
    assert!(box2.is_empty().expect("crash"));
    assert_eq!(0, box2.count_with_cursor().expect("crash"));

    box3.remove_all().expect("crash");
    assert!(box3.is_empty().expect("crash"));
    assert_eq!(0, box3.count_with_cursor().expect("crash"));

    // put then get, then clear
    {
        let mut e1 = f1.new_entity();
        e1.t_u16 = 0xFFF;

        let new_id = match box1.put(&mut e1) {
            Err(err) => panic!("{err}"),
            Ok(id) => id,
        };

        match box1.get(new_id) {
            Err(err) => panic!("{err}"),
            Ok(opt) => {
                assert_eq!(0xFFF, opt.unwrap().t_u16);
            }
        }
        box1.remove_all().expect("crash");
    }

    // put_many, get_many, get_all
    {
        let mut ids = match box1.put_many(vec![&mut f1.new_entity(), &mut f1.new_entity()]) {
            Err(err) => panic!("{err}"),
            Ok(ids) => ids,
        };

        ids.push(404);

        let objects = match box1.get_many(ids.as_slice()) {
            Err(err) => panic!("{err}"),
            Ok(v) => v,
        };

        assert!(objects[0].is_some());
        assert!(objects[1].is_some());
        assert!(objects[2].is_none());

        let all_objects = match box1.get_all() {
            Err(err) => panic!("{err}"),
            Ok(objs) => objs,
        };

        assert_eq!(2, all_objects.len());
    }

    // contains*, remove_*
    {
        box1.remove_all().expect("crash");

        let mut ids = match box1.put_many(vec![
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
        ]) {
            Err(e) => panic!("{e}"),
            Ok(ids) => ids,
        };

        match box1.contains_many(&ids) {
            Ok(v) => assert!(v.iter().all(|b| *b)),
            Err(e) => panic!("{e}"),
        }

        ids.push(404);

        match box1.contains_many(&ids) {
            Ok(v) => assert!(v.iter().any(|b| !*b)),
            Err(e) => panic!("{e}"),
        }

        assert_ne!(true, box1.contains(404).expect("crash"));

        if let Ok(r) = box1.remove_with_id(404) {
            assert_ne!(true, r);
        }

        // remove_many uses remove_with_id, so its transitively tested
        match box1.remove_many(&ids) {
            Ok(r) => assert_ne!(true, r[5]),
            Err(e) => panic!("{e}"),
        }

        assert!(box1.is_empty().expect("crash"));
    }
}
