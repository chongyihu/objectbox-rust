use crate::store::Store;
use crate::model::SchemaID;
use flatbuffers::FlatBufferBuilder;

pub trait FBOBBridge {
  fn to_fb(self, builder: &mut FlatBufferBuilder);

  // This is object-safe, but can't be dispatched on a (casted) trait object
  // fn from_FB(store: &mut store::Store, table: &Table) -> Self; // factory method
}

pub trait IdExt {
  fn get_id(&self) -> SchemaID;
  fn set_id(&mut self, id: SchemaID);
}

// TODO
/*
pub trait RelationExt {
  fn to_one_relation<T>(&self) -> T;

  /// Any is rust's dynamic type? If so, then the relation's type
  /// requires the related trait.
  fn to_many_relations(&self) -> Any
}
*/

// Reference from Store and Box with this type
pub trait OBBlanket: IdExt + FBOBBridge {}
impl<T> OBBlanket for T where T: IdExt + FBOBBridge {}

use flatbuffers::Table;

pub trait FactoryHelper<T: ?Sized> {
  fn make(&self, store: &mut Store, table: &mut Table) -> T;
}
pub struct Factory<T> { _required_for_generic_trait: Option<T> }

pub fn make_from_trait<T>(map: anymap::AnyMap, store: &mut Store, table: &mut Table)
-> Option<T> where T: 'static {
  if let Some(f) = map.get::<Box<dyn FactoryHelper<T>>>() {
    return Some(f.make(store, table));
  }
  None
}

#[cfg(test)]
#[test]
fn blanket_directly_applied_on_entity_type() {
  // imagine this were an external struct
  // from a different package / crate / module etc.

  struct SomeEntity {
    id: SchemaID
  }

  impl FBOBBridge for SomeEntity {
    fn to_fb(self, builder: &mut FlatBufferBuilder<'_>) {}

    // non-member method, static(?) factory function, can't dispatch on a trait
    // fn from_FB(store: &mut store::Store, table: &Table) -> Self {
    //   SomeEntity { id: 1 }
    // }
    // update: the from_fb function will be executed by the make_from_trait function
  }

  impl IdExt for SomeEntity {
    fn get_id(&self) -> SchemaID {
      self.id
    }
    fn set_id(&mut self, id: SchemaID) {
      self.id = id;
    }
  }

  // call trait method on original object
  let e0 = SomeEntity{ id: 1 };

  assert_eq!(e0.get_id(), 1);

  // single-owner boxed immutable
  let b1 = Box::new(SomeEntity { id: 3});
  let t1 = b1 as Box<dyn OBBlanket>;

  assert_eq!(t1.get_id(), 3);

  // borrowed mutable
  let e2 = &mut SomeEntity{ id: 5 };
  let m2 = e2 as &mut dyn OBBlanket;

  m2.set_id(5005);

  assert_eq!(m2.get_id(), 5005);

  // borrowed immutable
  let e3 = &SomeEntity{ id: 6 };
  let r3 = e3 as &dyn OBBlanket;

  assert_eq!(r3.get_id(), 6);
}

#[cfg(test)]
#[test]
fn entity_factories() {
  unsafe {
    struct Entity0 { id: SchemaID }
    struct Entity1 { id: SchemaID }
    struct Entity2 { id: SchemaID }

    impl FactoryHelper<Entity0> for Factory<Entity0> {
      fn make(&self, store: &mut Store, table: &mut Table) -> Entity0 {
          Entity0{ id: 0 }
      }
    }

    impl FactoryHelper<Entity1> for Factory<Entity1> {
      fn make(&self, store: &mut Store, table: &mut Table) -> Entity1 {
          Entity1{ id: 1 }
      }
    }

    impl FactoryHelper<Entity2> for Factory<Entity2> {
      fn make(&self, store: &mut Store, table: &mut Table) -> Entity2 {
          Entity2{ id: 2 }
      }
    }

    let store = &mut Store {
        model_callback: None,
    };

    let table = &Table::new(&[0u8], 0);

    // this should be const boxed where it is generated
    let f0 = Factory::<Entity0> { _required_for_generic_trait: None };
    let f1 = Factory::<Entity1> { _required_for_generic_trait: None };
    let f2 = Factory::<Entity2> { _required_for_generic_trait: None };

    let e0 = f0.make(store, table);
    let e1 = f1.make(store, table);
    let e2 = f2.make(store, table);

    assert_eq!(e0.id, 0);
    assert_eq!(e1.id, 1);
    assert_eq!(e2.id, 2);

    // AnyMap experiment
    {
      let mut map = anymap::AnyMap::new();

      map.insert(f0);
      map.insert(f1);
      map.insert(f2);

      let f0 = map.get::<Factory<Entity0>>();
      let f1 = map.get::<Factory<Entity1>>();
      let f2 = map.get::<Factory<Entity2>>();

      let e0 = f0.unwrap().make(store, table);
      let e1 = f1.unwrap().make(store, table);
      let e2 = f2.unwrap().make(store, table);
  
      assert_eq!(e0.id, 0);
      assert_eq!(e1.id, 1);
      assert_eq!(e2.id, 2);
    }

    // experiment boxed factories
    {
      let mut map = anymap::AnyMap::new();
      let f0 = Factory::<Entity0> { _required_for_generic_trait: None };
      
      map.insert(Box::new(f0) as Box<dyn FactoryHelper<Entity0>>);
      
      let e0 = make_from_trait::<Entity0>(map, store, table);
      assert_eq!(e0.is_some(), true); // \o/
    }

    // experiment ref'ed factories
    {
      fn make_from_ref<T>(map: anymap::AnyMap, store: &mut Store, table: &Table)
      -> Option<T> where T: 'static {
        if let Some(f) = map.get::<Factory<T>>() {
          // return f.make (nope, unknown trait)
        }
        None
      }

      let mut map = anymap::AnyMap::new();
      let f0: &'static Factory<Entity0> = &Factory::<Entity0> { _required_for_generic_trait: None };
      map.insert(f0);
      
      let e0 = make_from_ref::<Entity0>(map, store, table);
      assert_ne!(e0.is_some(), true); // :(
    }
  }
}


