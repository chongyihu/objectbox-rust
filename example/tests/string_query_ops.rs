use example::{
    make_factory_map, make_model, new_entity3_condition_factory, Entity, Entity2, Entity3,
    Entity3ConditionFactory,
};
use objectbox::{opt::Opt, store::Store, query::condition::Condition, error};

use serial_test::serial;

trait TesterExt {
    fn given_condition_count(&mut self, c: &mut Condition<Entity3>, i: usize) -> error::Result<()>;
}

impl TesterExt for objectbox::r#box::Box<'_, Entity3> {
    fn given_condition_count(&mut self, c: &mut Condition<Entity3>, i: usize) -> error::Result<()> {
        let q2 = self.query(c)?;
        let found_list = q2.find()?;
        assert_eq!(i, found_list.len());
        Ok(())
    }
}

#[test]
#[serial]
fn string_query_tests() -> error::Result<()> {
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

    // query builder, query condition, case sensitivity
    {
        let mut first = Entity3 {
            id: 1,
            hello: "world".to_string(),
        };
        let mut second = Entity3 {
            id: 2,
            hello: "real world".to_string(),
        };
        let mut third = Entity3 {
            id: 3,
            hello: "REAL world".to_string(),
        };
        let _ = box3.put(&mut first);
        let _ = box3.put(&mut second);
        let _ = box3.put(&mut third);
        let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

        // TODO FIXME: there's something clearly wrong here,
        // TODO maybe something with how spaces in &str are handled in rust
        // let mut c = hello.case_sensitive(true).and(hello.contains("real world"));
        // let q = box3.query(&mut c)?;
        // let found_list = q.find()?;
        // assert_eq!(2, found_list.len());
        // assert_eq!(first.hello, found_list[0].hello);

        // TODO FIXME: also broken
        // let mut c2 = hello.case_sensitive(true).and(hello.contains("real"));
        // let q2 = box3.query(&mut c2)?;
        // let found_list2 = q2.find()?;
        // assert_eq!(1, found_list2.len());

        // TODO FIXME: broken
        // let mut c3 = hello.case_sensitive(true).and(hello.in_strings(&vec!["world".to_string(), "does not exist".to_string(),]));
        // box3.given_condition_count(&mut c3, 1);
        // TODO FIXME: broken
        // let mut c4 = hello.any_equals("world");
        // box3.given_condition_count(&mut c4, 1);

        let mut c5 = hello.contains("world");
        box3.given_condition_count(&mut c5, 3)?;

        // TODO FIX LOGIC or implementation, always return 3
        // let mut c5_2 = hello.case_sensitive(false) & hello.contains("real");
        // box3.given_condition_count(&mut c5_2, 2);

        // let mut c6 = hello.contains_element("test");
        // let mut c7 = hello.contains_key_value("meh", "bleh");

        let mut c8 = hello.ends_with("d");
        box3.given_condition_count(&mut c8, 3)?;

        // TODO FIX LOGIC or implementation, always returns 3
        // let mut c8_2 = hello.ends_with(" world");
        // box3.given_condition_count(&mut c8_2, 2);

        // TODO FIX LOGIC or implementation, always returns 3
        // let mut c9_1 = hello.case_sensitive(true) & hello.starts_with("h");
        // let mut c9_2 = hello.starts_with("H");
        // let mut c9_3 = hello.starts_with("w");
        // box3.given_condition_count(&mut c9_1, 1);
        // box3.given_condition_count(&mut c9_2, 1);
        // box3.given_condition_count(&mut c9_3, 1);

        // TODO FIX LOGIC or implementation, always returns 3
        // let mut ca = hello.in_strings(&vec!["ea".to_string()]);
        // box3.given_condition_count(&mut ca, 3);
        // let mut cb = hello.eq("a".to_string());
        // let mut cc = hello.ne("a".to_string());
        // let mut cd = hello.lt("a".to_string());
        // let mut ce = hello.le("a".to_string());
        // let mut cf = hello.ge("a".to_string());
        // let mut d0 = hello.gt("a".to_string());
        //  and more...
    }

    Ok(())
}
