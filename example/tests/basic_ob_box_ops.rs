use example::{make_factory_map, make_model, Entity, Entity2, Entity3};
use objectbox::error;
use objectbox::traits::{self, IdExt};
use objectbox::{opt::Opt, store::Store};
use std::rc;

use serial_test::serial;

#[test]
#[serial]
fn test_box_put_and_count_and_remove_all() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;
    let mut box2 = store.get_box::<Entity2>()?;
    box2.remove_all()?;
    let mut box1 = store.get_box::<Entity>()?;
    box1.remove_all()?;

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

    box1.put(&mut e1)?;
    box2.put(&mut e2)?;
    box3.put(&mut e3)?;

    assert_eq!(false, e1.get_id() == 0, "Set new ID after put");
    assert_eq!(false, e2.get_id() == 0);
    assert_eq!(false, e3.get_id() == 0);

    assert_eq!(false, box1.is_empty()?);
    assert_eq!(false, box2.is_empty()?);
    assert_eq!(false, box3.is_empty()?);

    assert_eq!(1, box1.count()?);
    assert_eq!(1, box2.count()?);
    assert_eq!(1, box3.count()?);
    assert_eq!(1, box1.count_with_limit(1)?);
    assert_eq!(1, box2.count_with_limit(1)?);
    assert_eq!(1, box3.count_with_limit(1)?);
    assert_eq!(1, box1.count_with_cursor()?);
    assert_eq!(1, box2.count_with_cursor()?);
    assert_eq!(1, box3.count_with_cursor()?);

    box1.remove_all()?;
    assert!(box1.is_empty()?);
    assert_eq!(0, box1.count_with_cursor()?);

    box2.remove_all()?;
    assert!(box2.is_empty()?);
    assert_eq!(0, box2.count_with_cursor()?);

    box3.remove_all()?;
    assert!(box3.is_empty()?);
    assert_eq!(0, box3.count_with_cursor()?);

    // put then get, then clear
    {
        let mut e1 = f1.new_entity();
        e1.t_u16 = 0xFFF;

        let new_id = box1.put(&mut e1)?;
        assert_eq!(0xFFF, box1.get(new_id)?.unwrap().t_u16);
        box1.remove_all()?;
    }

    // put_many, get_many, get_all
    {
        let mut ids = box1.put_many(vec![&mut f1.new_entity(), &mut f1.new_entity()])?;

        ids.push(404);

        let objects = box1.get_many(ids.as_slice())?;

        assert!(objects[0].is_some());
        assert!(objects[1].is_some());
        assert!(objects[2].is_none());

        let all_objects = box1.get_all()?;

        assert_eq!(2, all_objects.len());
    }

    // contains*, remove_*
    {
        box1.remove_all()?;

        let mut ids = box1.put_many(vec![
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
            &mut f1.new_entity(),
        ])?;

        assert!(box1.contains_many(&ids)?.iter().all(|b| *b));

        ids.push(404);

        assert!(box1.contains_many(&ids)?.iter().any(|b| !*b));

        assert_ne!(true, box1.contains(404)?);

        assert_ne!(true, box1.remove_with_id(404)?);

        // remove_many uses remove_with_id, so its transitively tested
        assert_ne!(true, box1.remove_many(&ids)?[5]);

        assert!(box1.is_empty()?);
    }

    Ok(())
}
