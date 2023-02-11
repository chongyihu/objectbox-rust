#![allow(non_camel_case_types)]

use crate::{c::*, traits::OBBlanket};
use core::marker::PhantomData;
use std::rc::Rc;

use super::{condition::Condition, enums::ConditionOp};

// Idea: lock down what which ops are available given the generic param
// and the generated blanket.
// Collect the enums via the traits / blankets.
// Pass enums / tuples down to the builder.

pub type IdsAndType = Rc<(obx_schema_id, obx_schema_id, u8)>;

// Don't overcomplicate the generic params, because blankets
// depend on these, and it causes too much syntactical noise
pub struct ConditionBuilder<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    // entity_id: obx_schema_id, property_id: obx_schema_id, property_type: u8,
    ids_and_type: IdsAndType,
    order_flags: u32,
}

impl<Entity: OBBlanket> ConditionBuilder<Entity> {
    fn get_property_attrs(&self) -> IdsAndType {
        self.ids_and_type.clone()
    }

    fn order_flags(&mut self, of: u32) -> &Self {
        self.order_flags = of;
        self
    }

    // TODO turn on when there is support for Option<*> properties
    /*
    pub fn is_null(&self) -> Condition<Entity> {
        Condition::new(self.ids_and_type.clone(), IsNull)
    }
    pub fn is_not_null(&self) -> Condition<Entity> {
        Condition::new(self.ids_and_type.clone(), NotNull)
    }
    */
}

// TODO figure out if std::ops really doesn't contain <, >, <=, >=
// If op overloading has to be thru, the std::cmp::Partial{Ord,Eq}
// then no op overloading, Because every op return type is bool.
pub trait EqExt<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: Rhs) -> Condition<Entity>;
    fn ne(&self, other: Rhs) -> Condition<Entity>;
}

pub trait OrdExt<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn lt(&self, other: Rhs) -> Condition<Entity>;
    fn gt(&self, other: Rhs) -> Condition<Entity>;
    fn le(&self, other: Rhs) -> Condition<Entity>;
    fn ge(&self, other: Rhs) -> Condition<Entity>;
}

pub trait StringExt<Entity: OBBlanket> {
    fn contains(s: &str) -> Condition<Entity>;
    fn contains_element(s: &str) -> Condition<Entity>;
    // contains_key_value_string // huh?
    fn starts_with(s: &str) -> Condition<Entity>;
    fn ends_with(s: &str) -> Condition<Entity>;
    // fn in_strings(&[&str]) -> Condition; // not sure about the name
    fn any_equals(list: &[&str]) -> Condition<Entity>; // not sure about the input type
    fn case_sensitive(b: bool) -> Self;
}

// TODO blanket later in code_gen

pub trait BetweenExt<Entity: OBBlanket, SurroundType>
where
    SurroundType: ?Sized,
{
    fn between(&self, this: SurroundType, that: SurroundType) -> Condition<Entity>;
}

pub trait InOutExt<Entity: OBBlanket, U>
where
    U: Sized,
{
    fn member_of(&self, vec: Vec<U>) -> Condition<Entity>;
    fn not_member_of(&self, vec: Vec<U>) -> Condition<Entity>;
}

impl<Entity: OBBlanket> EqExt<Entity, i64> for ConditionBuilder<Entity> {
    fn eq(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_i64(other))
    }
    fn ne(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_i64(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, i64> for ConditionBuilder<Entity> {
    fn lt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_i64(other))
    }
    fn gt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_i64(other))
    }
    fn le(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_i64(other))
    }
    fn ge(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_i64(other))
    }
}

impl<Entity: OBBlanket> EqExt<Entity, f64> for ConditionBuilder<Entity> {
    fn eq(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_f64(other))
    }
    fn ne(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_f64(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, f64> for ConditionBuilder<Entity> {
    fn lt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_f64(other))
    }
    fn gt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_f64(other))
    }
    fn le(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_f64(other))
    }
    fn ge(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_f64(other))
    }
}

impl<Entity: OBBlanket> EqExt<Entity, String> for ConditionBuilder<Entity> {
    fn eq(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_string(other))
    }
    fn ne(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_string(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, String> for ConditionBuilder<Entity> {
    fn lt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_string(other))
    }
    fn gt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_string(other))
    }
    fn le(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_string(other))
    }
    fn ge(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_string(other))
    }
}

impl<Entity: OBBlanket> EqExt<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn eq(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_vecu8(other))
    }
    fn ne(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NoOp)
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn lt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_vecu8(other))
    }
    fn gt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_vecu8(other))
    }
    fn le(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_vecu8(other))
    }
    fn ge(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_vecu8(other))
    }
}
impl<Entity: OBBlanket> EqExt<Entity, Vec<String>> for ConditionBuilder<Entity> {
    fn eq(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_vecstring(other))
    }
    fn ne(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_vecstring(other))
    }
}
impl<Entity: OBBlanket> OrdExt<Entity, Vec<String>> for ConditionBuilder<Entity> {
    fn lt(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_vecstring(other))
    }
    fn gt(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_vecstring(other))
    }
    fn le(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_vecstring(other))
    }
    fn ge(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_vecstring(other))
    }
}
impl<Entity: OBBlanket> BetweenExt<Entity, i64> for ConditionBuilder<Entity> {
    fn between(&self, this: i64, that: i64) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::Between_i64(this, that),
        )
    }
}
impl<Entity: OBBlanket> BetweenExt<Entity, f64> for ConditionBuilder<Entity> {
    fn between(&self, this: f64, that: f64) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::Between_f64(this, that),
        )
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, i32> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_i32(vec))
    }
    fn not_member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NotIn_i32(vec))
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, i64> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_i64(vec))
    }
    fn not_member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NotIn_i64(vec))
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, String> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_String(vec))
    }
    fn not_member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NoOp)
    }
}

/// Blankets
pub trait BoolBlanket<Entity: OBBlanket>:
    EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}


pub trait CharBlanket<Entity: OBBlanket>:
    EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}


pub trait I8Blanket<Entity: OBBlanket>:
    EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}

pub trait U8Blanket<Entity: OBBlanket>:
    EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}

pub trait F32Blanket<Entity: OBBlanket>: OrdExt<Entity, i64> + BetweenExt<Entity, i64> {}
pub trait F64Blanket<Entity: OBBlanket>: OrdExt<Entity, i64> + BetweenExt<Entity, i64> {}

pub trait I16Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
pub trait U16Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}

pub trait I32Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
pub trait U32Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}

pub trait I64Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i64> {}
pub trait U64Blanket<Entity: OBBlanket>: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i64> {}

pub trait VecU8Blanket<Entity: OBBlanket>: EqExt<Entity, Vec<u8>> + OrdExt<Entity, Vec<u8>> {}
pub trait VecStringBlanket<Entity: OBBlanket>: EqExt<Entity, Vec<String>> + OrdExt<Entity, Vec<String>> {}
pub trait StringBlanket<Entity: OBBlanket>: EqExt<Entity, String> + OrdExt<Entity, String> + BetweenExt<Entity, String> + InOutExt<Entity, String> {}

impl<Entity: OBBlanket> F32Blanket<Entity> for Entity where Entity: OrdExt<Entity, i64> + BetweenExt<Entity, i64> {}
impl<Entity: OBBlanket> F64Blanket<Entity> for Entity where Entity: OrdExt<Entity, i64> + BetweenExt<Entity, i64> {}
impl<Entity: OBBlanket> BoolBlanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> CharBlanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}        
impl<Entity: OBBlanket> I8Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> U8Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> I16Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> U16Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> I32Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> U32Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i32> {}
impl<Entity: OBBlanket> I64Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i64> {}
impl<Entity: OBBlanket> U64Blanket<Entity> for Entity where Entity: EqExt<Entity, i64> + OrdExt<Entity, i64> + BetweenExt<Entity, i64> + InOutExt<Entity, i64> {}
impl<Entity: OBBlanket> VecU8Blanket<Entity> for Entity where Entity: EqExt<Entity, Vec<u8>> + OrdExt<Entity, Vec<u8>> {}
impl<Entity: OBBlanket> VecStringBlanket<Entity> for Entity where Entity: EqExt<Entity, Vec<String>> + OrdExt<Entity, Vec<String>> {}
impl<Entity: OBBlanket> StringBlanket<Entity> for Entity where Entity: EqExt<Entity, String> + OrdExt<Entity, String> + BetweenExt<Entity, String> + InOutExt<Entity, String> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_impl_test() {}
}
