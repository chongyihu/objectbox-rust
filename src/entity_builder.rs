use crate::c::{OBXPropertyFlags, OBXPropertyType};
use crate::model::{SchemaID, SchemaUID};

pub struct Entity {
  name: String,
  id: SchemaID,
  uid: SchemaUID,
  properties: Vec<Property>,
}


pub struct Property {
  name: String,
  id: SchemaID,
  uid: SchemaUID,
  typ: OBXPropertyType,
  flags: OBXPropertyFlags
}

pub struct EntityBuilder {
  pub entities: Vec<Entity>,
}

impl EntityBuilder {
  pub fn new() -> EntityBuilder {
      EntityBuilder {
          entities: Vec::new(),
      }
  }

  pub fn add_entity(&mut self, name: &str, id: SchemaID, uid: SchemaUID) -> &mut Self {
      let entity = Entity {
          name: name.to_string(),
          properties: Vec::new(),
          id,
          uid
      };
      self.entities.push(entity);
      self
  }

  pub fn add_property(&mut self, name: &str, id: SchemaID, uid: SchemaUID, typ: OBXPropertyType, flags: OBXPropertyFlags) -> &mut Self {
      let property = Property {
          name: name.to_string(),
          id,
          uid,
          typ,
          flags
      };
      let last = &mut self.entities.last_mut();
      match last {
          Some(e) => {
              let properties = &mut e.properties;
              properties.push(property)
          },
          _ => ()
      }
      self
  }
}