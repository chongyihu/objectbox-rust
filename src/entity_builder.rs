use crate::c::{self, OBXPropertyFlags, OBXPropertyType};

pub(crate) struct Entity {
    name: String,
    id: c::obx_schema_id,
    uid: c::obx_uid,
    pub properties: Vec<Property>,
}

pub(crate) struct Property {
    name: String,
    id: c::obx_schema_id,
    uid: c::obx_uid,
    typ: OBXPropertyType,
    flags: OBXPropertyFlags,
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

    pub(crate) fn add_entity(
        &mut self,
        name: &str,
        id: c::obx_schema_id,
        uid: c::obx_uid,
    ) -> &mut Self {
        let entity = Entity {
            name: name.to_string(),
            properties: Vec::new(),
            id,
            uid,
        };
        self.entities.push(entity);
        self
    }

    pub(crate) fn add_property(
        &mut self,
        name: &str,
        id: c::obx_schema_id,
        uid: c::obx_uid,
        typ: OBXPropertyType,
        flags: OBXPropertyFlags,
    ) -> &mut Self {
        let (new_type, new_flags) = if typ == c::OBXPropertyType_Char {
            (
                c::OBXPropertyType_Char,
                flags & c::OBXPropertyFlags_UNSIGNED,
            )
        } else {
            (typ, flags)
        };
        let property = Property {
            name: name.to_string(),
            id,
            uid,
            typ: new_type,
            flags: new_flags,
        };
        let last = &mut self.entities.last_mut();
        match last {
            Some(e) => {
                let properties = &mut e.properties;
                properties.push(property)
            }
            _ => (),
        }
        self
    }
}
