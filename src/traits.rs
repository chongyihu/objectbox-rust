#[path = "./store.rs"]
pub mod store;

#[path = "./model.rs"]
mod model;

pub trait FBOBBridge {
  fn to_fb(self /* TODO, builder: &fb.Builder */);

  // This is object-safe, but can't be dispatched on a (casted) trait object
  // fn from_FB(store: &mut store::Store, byte_buffer: &ByteBuffer) -> Self; // factory method
}

pub trait IdExt {
  fn get_id(&self) -> model::SchemaID;
  fn set_id(&mut self, id: model::SchemaID);
}

// TODO determine if we need an ext trait for determining the TypeId,
// the compiler should warn if an entity type doesn't 

// TODO
/*
pub trait RelationExt {
  fn to_one_relation<T>(&self) -> T;

  /// Any is rust's dynamic type? If so, then the relation's type
  /// requires the related trait.
  fn to_many_relations(&self) -> Any
}
*/

trait OBBlanket: IdExt + FBOBBridge {}
impl<T> OBBlanket for T where T: IdExt + FBOBBridge {}

#[cfg(test)]
#[test]
fn tuck_blanket() {
  // imagine this were an external struct
  // from a different package / crate / module etc.

  struct SomeEntity {
    id: model::SchemaID
  }

  impl FBOBBridge for SomeEntity {
    fn to_fb(self /* TODO, builder: &fb.Builder */) {}

    // non-member method, static(?) factory function, can't dispatch on a trait
    // fn from_FB(store: &mut store::Store, byte_buffer: &ByteBuffer) -> Self {
    //   SomeEntity { id: 1 }
    // }
  }

  impl IdExt for SomeEntity {
    fn get_id(&self) -> model::SchemaID {
      self.id
    }
    fn set_id(&mut self, id: model::SchemaID) {
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


use bytebuffer::ByteBuffer;
use std::any::Any;


#[cfg(test)]
#[test]
fn build_factories_with_closures_nope() {
  use std::{collections::HashMap, any::TypeId};

  // won't work, because Store and Box need to know the enum
  // ahead of time
  {
    struct EntityA { id: model::SchemaID }
    struct EntityB { id: model::SchemaID }

    enum EntityTypes {
      EntityA(Box<fn() -> EntityA>),
      EntityB(Box<fn() -> EntityB>),
    }

    let a = EntityTypes::EntityA(Box::new(|| EntityA { id: 0 } ));
    let b = EntityTypes::EntityB(Box::new(|| EntityB { id: 1 } ));

    let mut map = HashMap::<model::SchemaID, EntityTypes>::new();

  }

  // {
  //   trait Factory<T: ?Sized> {
  //     fn make(&self, store: &mut store::Store, byte_buffer: &ByteBuffer) -> T;
  //   }

  //   let mut map = HashMap::<u32, Box<dyn Factory<T>>>::new();

  //   struct Entity0 { id: model::SchemaID }
  //   struct Factory0;

  //   struct Entity1 { id: model::SchemaID }
  //   struct Factory1;

  //   impl Factory<Entity0> for Factory0 {
  //     fn make(&self, store: &mut store::Store, byte_buffer: &ByteBuffer) -> Entity0 {
  //         Entity0{ id: 0 }
  //     }
  //   }
  //   impl Factory<Entity1> for Factory1 {
  //     fn make(&self, store: &mut store::Store, byte_buffer: &ByteBuffer) -> Entity1 {
  //       Entity1{ id: 1 }
  //   }
  // }

  //   map.insert(0, Box::new(Factory0{}));
  //   map.insert(1, Box::new(Factory1{}));

  //   let store = store::Store {};
  //   let byte_buffer = ByteBuffer::new();
  //   let e0 = map.get(0).unwrap().make(&mut store, &byte_buffer) as Entity0;
  //   let e1 = map.get(1).unwrap().make(&mut store, &byte_buffer) as Entity1;
  //   assert_eq!(e0.id, 0);
  //   assert_eq!(e1.id, 1);
  // }

  // {
  //   struct Entity0 { id: model::SchemaID }
  //   struct Entity1 { id: model::SchemaID }

  //   let mut map = HashMap::<model::SchemaID, Box<dyn Fn(&mut store::Store, &ByteBuffer) -> dyn Any>>::new();
  //   map.insert(0, Box::new(|store, bb| Entity0 { id : 0}) as Box<dyn Any>);
  //   map.insert(1, Box::new(|store, bb| Entity1 { id : 1}) as Box<dyn Any>);
  //   let store = store::Store {};
  //   let byte_buffer = ByteBuffer::new();
  //   let e0 = map.get(0).unwrap()(&mut store, &byte_buffer) as Entity0;
  //   let e1 = map.get(1).unwrap()(&mut store, &byte_buffer) as Entity1;
  //   assert_eq!(e0.id, 0);
  //   assert_eq!(e1.id, 1);
  // }
}


// struct Factory<T> {
//     pub maker: fn(store: &mut store::Store, byte_buffer: &ByteBuffer) -> T,
// }

// #[cfg(test)]
// #[test]
// fn build_factories_with_closures_nope() {
//   use std::collections::HashMap;
//   let mut map = HashMap::new();

//   let f1 = Factory {
//     maker: |s, bb| String::from("new"),
//   };

//   let f2 = Factory {
//     maker: |s, bb| 32_u32,
//   };

//   // map.insert(
//   //   0_u32,
//   //   Box::new(f1),
//   // );

//   map.insert(
//     1_u32,
//     Box::new(f2),
//   );

//   // let result0 = map.get(&0_u32).unwrap();
//   let result1 = map.get(&1_u32).unwrap();

//   let store = &mut store::Store {};
//   let bb = bytebuffer::ByteBuffer::new();

//   // assert_eq!((result0.builder)(store, &bb).as_str(), "new");
//   assert_eq!((result1.maker)(store, &bb), 32_u32);
// }
