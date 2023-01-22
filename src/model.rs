use crate::{entity_builder::EntityBuilder, c, error::Error};
use std::{ffi, ptr};

pub type SchemaID = u32;
pub type SchemaUID = u64;

/// Model is used to define a database model. Use as a fluent interface (builder pattern)
pub struct Model {
    c_ptr: *mut c::OBX_model,
    error: Option<Error>,
    builder: Box<EntityBuilder>
}

impl Model {
    pub fn new(builder: Box<EntityBuilder>) -> Self {
        match c::new_mut(unsafe { c::obx_model() }) {
            Ok(c_ptr) => Model { c_ptr, error: None, builder },
            Err(e) => Model {
                c_ptr: ptr::null_mut(),
                error: Some(e),
                builder
            },
        }
    }

    /// Create an entity.
    pub fn entity(mut self, name: &str, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();
            self.error =
                c::call(unsafe { c::obx_model_entity(self.c_ptr, c_name.as_ptr(), id, uid) }).err();
        }
        self.builder.as_mut().add_entity(name, id, uid);
        self
    }

    /// Inform the model about the last entity that was ever defined in the model.
    pub fn last_entity_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_entity_id(self.c_ptr, id, uid) }
        }
        self
    }

    /// Inform the model about the last index that was ever defined in the model.
    pub fn last_index_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_index_id(self.c_ptr, id, uid) }
        }
        self
    }

    /// Inform the model about the last relation that was ever defined in the model.
    pub fn last_relation_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_relation_id(self.c_ptr, id, uid) }
        }
        self
    }

    /// Inform the model about the last property that was ever defined on the entity.
    /// Finishes building the entity, returning the parent Model.
    pub fn last_property_id(mut self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_entity_last_property_id(self.c_ptr, id, uid) })
                    .err();
        }

        self
    }

    /// Create a property.
    pub fn property(
        mut self,
        name: &str,
        id: SchemaID,
        uid: SchemaUID,
        // type === typedef, is a reserved keyword, intentional
        typ: c::OBXPropertyType,
        flags: c::OBXPropertyFlags,
    ) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();
            self.error = c::call(unsafe {
                c::obx_model_property(self.c_ptr, c_name.as_ptr(), typ, id, uid)
            })
            .err();
        }

        if flags > 0 && self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_property_flags(self.c_ptr, flags) }).err();
        }

        self.builder.as_mut().add_property(name, id, uid, typ, flags);

        self
    }

    /// Declare an index on the last created property.
    pub fn property_index(mut self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_property_index_id(self.c_ptr, id, uid) }).err();
        }
        self
    }

    /// Declare a to-one relation on the last created property.
    /// No need to declare the index separately using property_index(), it's created automatically.
    pub fn property_relation(
        mut self,
        target_entity_name: &str,
        index_id: SchemaID,
        index_uid: SchemaUID,
    ) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(target_entity_name).unwrap();
            self.error = c::call(unsafe {
                c::obx_model_property_relation(
                    self.c_ptr,
                    c_name.as_ptr(),
                    index_id,
                    index_uid,
                )
            })
            .err();
        }
        self
    }

    /// Declare a standalone to-many relation between this entity and another one
    pub fn relation(
        mut self,
        relation_id: SchemaID,
        relation_uid: SchemaUID,
        target_entity_id: SchemaID,
        target_entity_uid: SchemaUID,
    ) -> Self {
        if self.error.is_none() {
            self.error = c::call(unsafe {
                c::obx_model_relation(
                    self.c_ptr,
                    relation_id,
                    relation_uid,
                    target_entity_id,
                    target_entity_uid,
                )
            })
            .err();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_builder_positive() {
        let builder = Box::new(EntityBuilder::new());
        let model = Model::new(builder)
            .entity("A", 1, 1)
            .property(
                "id",
                1,
                101,
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
            )
            .property("text", 2, 202, c::OBXPropertyType_String, 102)
            .property_index(1, 101)
            .last_property_id(2, 202)
            .entity("B", 2, 2)
            .property(
                "id",
                1,
                301,
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
            )
            .property("number", 2, 202, c::OBXPropertyType_Int, 0)
            .last_property_id(2, 202)
            .last_entity_id(2, 2)
            .last_index_id(1, 301);

        assert!(model.error.is_none());
        assert_eq!(model.builder.entities.len(), 2);
        assert_eq!(model.builder.entities.first().unwrap().properties.len(), 2);
    }

    #[test]
    fn model_builder_negative() {
        let builder = Box::new(EntityBuilder::new());
        let model = Model::new(builder).entity("A", 1, 1).last_property_id(0, 0);

        let expected_err = format!(
            "{} Argument condition \"property_id\" not met",
            c::OBX_ERROR_ILLEGAL_ARGUMENT
        );
        let actual_err = format!("{}", model.error.unwrap());
        println!("expected: {}", &expected_err);
        println!("actual: {}", &actual_err);
        assert!(actual_err.starts_with(&expected_err));
    }
}
