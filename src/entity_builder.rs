use crate::c::{OBXPropertyFlags, OBXPropertyType, self};
use crate::model::{SchemaID, SchemaUID};

pub(crate) struct Entity {
  name: String,
  id: SchemaID,
  uid: SchemaUID,
  pub properties: Vec<Property>,
}


pub(crate) struct Property {
  name: String,
  id: SchemaID,
  uid: SchemaUID,
  typ: OBXPropertyType,
  flags: OBXPropertyFlags
}

pub struct EntityBuilder {
  pub(crate) entities: Vec<Entity>,
}

impl EntityBuilder {
  pub fn new() -> EntityBuilder {
      EntityBuilder {
          entities: Vec::new(),
      }
  }

  pub(crate) fn add_entity(&mut self, name: &str, id: SchemaID, uid: SchemaUID) -> &mut Self {
      let entity = Entity {
          name: name.to_string(),
          properties: Vec::new(),
          id,
          uid
      };
      self.entities.push(entity);
      self
  }

  pub(crate) fn add_property(&mut self, name: &str, id: SchemaID, uid: SchemaUID, typ: OBXPropertyType, flags: OBXPropertyFlags) -> &mut Self {
      // let property = 
      //   Property {
      //     name: name.to_string(),
      //     id,
      //     uid,
      //     typ,
      //     flags
      //   };
      // On the OBX side of things, pretend char is u32
      let property = if typ == c::OBXPropertyType_Char {
        Property {
          name: name.to_string(),
          id,
          uid,
          typ,
          flags
        }
      } else {
        // don't actually use char on ob
        Property {
          name: name.to_string(),
          id,
          uid,
          typ: c::OBXPropertyType_Int,
          flags: flags | c::OBXOrderFlags_UNSIGNED
        }
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