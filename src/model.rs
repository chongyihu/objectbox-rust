#[allow(dead_code)]

use crate::{entity_builder::EntityBuilder, c, error::Error};
use std::{ffi, ptr};


pub type SchemaID = u32;
pub type SchemaUID = u64;

/// Model is used to define a database model. Use as a fluent interface (builder pattern)
pub struct Model {
    pub(crate) obx_model: *mut c::OBX_model,
    error: Option<Error>,
    builder: Box<EntityBuilder>
}

impl Drop for Model {
    fn drop(&mut self) {
      if !self.obx_model.is_null() {
        self.error = c::call(unsafe { c::obx_model_free(self.obx_model) }).err();
        self.obx_model = std::ptr::null_mut();
      }

      if let Some(err) = &self.error {
        eprintln!("Error: {err}");
      }
    }
  }

impl Model {
    pub fn new(builder: Box<EntityBuilder>) -> Self {
        match c::new_mut(unsafe { c::obx_model() }) {
            Ok(c_ptr) => Model { obx_model: c_ptr, error: None, builder },
            Err(e) => Model {
                obx_model: ptr::null_mut(),
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
                c::call(unsafe { c::obx_model_entity(self.obx_model, c_name.as_ptr(), id, uid) }).err();
        }
        self.builder.as_mut().add_entity(name, id, uid);
        self
    }

    /// Inform the model about the last entity that was ever defined in the model.
    pub fn last_entity_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_entity_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last index that was ever defined in the model.
    pub fn last_index_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_index_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last relation that was ever defined in the model.
    pub fn last_relation_id(self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_relation_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last property that was ever defined on the entity.
    /// Finishes building the entity, returning the parent Model.
    pub fn last_property_id(mut self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_entity_last_property_id(self.obx_model, id, uid) })
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

            let (new_type, new_flags) = if typ == c::OBXPropertyType_Char {
                (c::OBXPropertyType_Int, flags & c::OBXPropertyFlags_UNSIGNED)
            }else {
                (typ, flags)
            };

            self.error = c::call(unsafe {
                    c::obx_model_property(self.obx_model, c_name.as_ptr(), new_type, id, uid)
                })
                .err();

            self.error = c::call(unsafe {
                    c::obx_model_property_flags(self.obx_model, new_flags)
                })
                .err()
        }

        self.builder.as_mut().add_property(name, id, uid, typ, flags);

        self
    }

    /// Declare an index on the last created property.
    pub fn property_index(mut self, id: SchemaID, uid: SchemaUID) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_property_index_id(self.obx_model, id, uid) }).err();
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
                    self.obx_model,
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
                    self.obx_model,
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
    fn big_model_test() {
        let builder = Box::new(EntityBuilder::new());
        let model = Model::new(builder)
            .entity("Entity3", 3, 7970053520278932057)
            .property("id", 1, 2787983164621428925, 5, 8193)
            .property_index(1, 2787983164621428925)
            .last_property_id(1, 2787983164621428925)
            .entity("Entity2", 2, 11822673736957101631)
            .property("id", 1, 5991353639039331047, 5, 8193)
            .property("index", 2, 3089634329043241515, 6, 8200)
            .property_index(1, 5991353639039331047)
            .last_property_id(2, 3089634329043241515)
            .entity("Entity", 1, 11761132493123297625)
            .property("id", 1, 3337858750878930464, 5, 8193)
            .property("index", 2, 2899896242679282690, 6, 8200)
            .property("t_bool", 3, 568698003315437374, 1, 0)
            .property("t_u8", 4, 13715947038748179573, 2, 8192)
            .property("t_i8", 5, 12371495807681136757, 2, 0)
            .property("t_i16", 6, 12826057009448917551, 3, 0)
            .property("t_u16", 7, 16359736789208522050, 3, 8192)
            .property("t_i32", 8, 3332525431949437605, 5, 32)
            .property("t_u32", 9, 13749208938569458861, 5, 8192)
            .property("t_u64", 10, 16701073851952767148, 6, 8192)
            .property("t_i64", 11, 3441224032049733712, 6, 0)
            .property("t_f32", 12, 2307762524769727799, 7, 0)
            .property("t_f64", 13, 8741798588134039250, 8, 0)
            .property("t_string", 14, 17661680862988529738, 9, 0)
            .property("t_char", 15, 8866068856020898908, 4, 0)
            .property("t_vec_string", 16, 6709516815320029775, 30, 0)
            .property("t_vec_bytes", 17, 475363337853790328, 23, 0)
            .property_index(1, 3337858750878930464)
            .last_property_id(17, 475363337853790328)
            .last_entity_id(1, 11761132493123297625)
            .last_index_id(1, 3337858750878930464);

        assert!(model.error.is_none());
    }

    #[test]
    fn model_builder_negative() {
        let builder = Box::new(EntityBuilder::new());
        let model = Model::new(builder).entity("A", 1, 1).last_property_id(0, 0);

        let expected_err = format!(
            "{} {} Argument condition \"property_id\" not met",
            c::OBX_ERROR_ILLEGAL_ARGUMENT, 0
        );

        let actual_err = format!("{}", model.error.as_ref().unwrap());
        println!("expected: {}", &expected_err);
        println!("actual: {}", &actual_err);
        assert!(actual_err.starts_with(&expected_err));
    }
}
