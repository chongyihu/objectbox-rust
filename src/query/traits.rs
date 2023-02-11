#![allow(non_camel_case_types)]

use crate::{c::*, traits::OBBlanket};
use core::marker::PhantomData;
use std::rc::Rc;

// Idea: lock down what which ops are available given the generic param
// and the generated blanket.
// Collect the enums via the traits / blankets.
// Pass enums / tuples down to the builder.

pub type IdsAndType = Rc<(obx_schema_id, obx_schema_id, u8)>;

include!("./enums.rs");
include!("./condition.rs");

// Don't overcomplicate the generic params, because blankets
// depend on these, and it causes too much syntactical noise
pub struct ConditionBuilder<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    // entity_id: obx_schema_id, property_id: obx_schema_id, property_type: u8,
    ids_and_type: IdsAndType,
}

impl<Entity: OBBlanket> ConditionBuilder<Entity> {
    fn get_property_attrs(&self) -> IdsAndType {
        self.ids_and_type.clone()
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
pub trait Eq<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: Rhs) -> Condition<Entity>;
    fn ne(&self, other: Rhs) -> Condition<Entity>;
}

pub trait Ord<Entity: OBBlanket, Rhs>
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

// TODO blanket later

trait BetweenExt<Entity: OBBlanket, SurroundType>
where
    SurroundType: ?Sized,
{
    fn between(&self, this: SurroundType, that: SurroundType) -> Condition<Entity>; 
}

trait InOutExt<Entity: OBBlanket, U>
where
    U: Sized,
{
    fn member_of(&self, vec: Vec<U>) -> Condition<Entity>;
    fn not_member_of(&self, vec: Vec<U>) -> Condition<Entity>;
}

impl<Entity: OBBlanket> Eq<Entity, i64> for ConditionBuilder<Entity> {
    fn eq(&self, other: i64) -> Condition<Entity> {
        Condition::new_group(Eq_i64(other))
    }
    fn ne(&self, other: i64) -> Condition<Entity> {
        Condition::new_group(Ne_i64(other))
    }
}

impl<Entity: OBBlanket> Ord<Entity, i64> for ConditionBuilder<Entity> {
    fn lt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Lt_i64(other))
    }
    fn gt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Gt_i64(other))
    }
    fn le(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Le_i64(other))
    }
    fn ge(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Ge_i64(other))
    }
}

impl<Entity: OBBlanket> Eq<Entity, f64> for ConditionBuilder<Entity> {
    fn eq(&self, other: f64) -> Condition<Entity> {
        Condition::new_group(Eq_f64(other))
    }
    fn ne(&self, other: f64) -> Condition<Entity> {
        Condition::new_group(Ne_f64(other))
    }
}

impl<Entity: OBBlanket> Ord<Entity, f64> for ConditionBuilder<Entity> {
    fn lt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Lt_f64(other))
    }
    fn gt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Gt_f64(other))
    }
    fn le(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Le_f64(other))
    }
    fn ge(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Ge_f64(other))
    }
}

impl<Entity: OBBlanket> Eq<Entity, String> for ConditionBuilder<Entity> {
    fn eq(&self, other: String) -> Condition<Entity> {
        Condition::new_group(Eq_string(other))
    }
    fn ne(&self, other: String) -> Condition<Entity> {
        Condition::new_group(Ne_string(other))
    }
}

impl<Entity: OBBlanket> Ord<Entity, String> for ConditionBuilder<Entity> {
    fn lt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Lt_string(other))
    }
    fn gt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Gt_string(other))
    }
    fn le(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Le_string(other))
    }
    fn ge(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Ge_string(other))
    }
}

impl<Entity: OBBlanket> Eq<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn eq(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new_group(Eq_vecu8(other))
    }
    fn ne(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new_group(Ne_vecu8(other))
    }
}

fn Eq_i64(_: i64) {}
fn Ne_i64(_: i64) {}
fn Lt_i64(_: i64) {}
fn Gt_i64(_: i64) {}
fn Le_i64(_: i64) {}
fn Ge_i64(_: i64) {}
fn Eq_f64(_: f64) {}
fn Ne_f64(_: f64) {}
fn Lt_f64(_: f64) {}
fn Gt_f64(_: f64) {}
fn Le_f64(_: f64) {}
fn Ge_f64(_: f64) {}
fn Eq_string(_: String) {}
fn Ne_string(_: String) {}
fn Lt_string(_: String) {}
fn Gt_string(_: String) {}
fn Le_string(_: String) {}
fn Ge_string(_: String) {}
fn Eq_vecu8(_: Vec<u8>) {}
fn Ne_vecu8(_: Vec<u8>) {}
fn Lt_vecu8(_: Vec<u8>) {}
fn Gt_vecu8(_: Vec<u8>) {}
fn Le_vecu8(_: Vec<u8>) {}
fn Ge_vecu8(_: Vec<u8>) {}
impl<Entity: OBBlanket> Ord<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn lt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Lt_vecu8(other))
    }
    fn gt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Gt_vecu8(other))
    }
    fn le(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Le_vecu8(other))
    }
    fn ge(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Ge_vecu8(other))
    }
}
fn Eq_vecstring(_: Vec<String>) {}
fn Ne_vecstring(_: Vec<String>) {}
impl<Entity: OBBlanket> Eq<Entity, Vec<String>> for ConditionBuilder<Entity> {
    fn eq(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new_group(Eq_vecstring(other))
    }
    fn ne(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new_group(Ne_vecstring(other))
    }
}
fn Lt_vecstring(_: Vec<String>) {}
fn Gt_vecstring(_: Vec<String>) {}
fn Le_vecstring(_: Vec<String>) {}
fn Ge_vecstring(_: Vec<String>) {}
impl<Entity: OBBlanket> Ord<Entity, Vec<String>> for ConditionBuilder<Entity> {
    fn lt(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Lt_vecstring(other))
    }
    fn gt(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Gt_vecstring(other))
    }
    fn le(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Le_vecstring(other))
    }
    fn ge(&self, other: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Ge_vecstring(other))
    }
}
fn Between_i64(_: i64, _: i64) {}
impl<Entity: OBBlanket> BetweenExt<Entity, i64> for ConditionBuilder<Entity> {
    fn between(&self, this: i64, that: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Between_i64(this, that))
    }
}
fn Between_f64(_: f64, _: f64) {}
impl<Entity: OBBlanket> BetweenExt<Entity, f64> for ConditionBuilder<Entity> {
    fn between(&self, this: f64, that: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), Between_f64(this, that))
    }
}
fn In_i32(_: i32) {}
fn NotIn_i32(_: i32) {}
impl<Entity: OBBlanket> InOutExt<Entity, i32> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), In_i32(vec))
    }
    fn not_member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), NotIn_i32(vec))
    }
}
fn In_i64(_: i64) {}
fn NotIn_i64(_: i64) {}
impl<Entity: OBBlanket> InOutExt<Entity, i64> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), In_i64(vec))
    }
    fn not_member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), NotIn_i64(vec))
    }
}
fn In_String(_: String) {}
fn NotIn_String(_: String) {}
impl<Entity: OBBlanket> InOutExt<Entity, String> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), In_String(vec))
    }
    fn not_member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), NotIn_String(vec))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::traits::ConditionOp::*;
/*
    impl<Entity: OBBlanket> Eq<Entity, u8> for ConditionBuilder<Entity> {
        fn eq(&self, other: u8) -> Condition<Entity> {
            Condition::new_group(eq_u8(other))
        }

        fn ne(&self, other: u8) -> Condition<Entity> {
            Condition::new_group(eq_u8(other))
        }
    }

    impl<Entity: OBBlanket> Ord<Entity, u8> for ConditionBuilder<Entity> {
        fn lt(&self, other: u8) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), lt_u8(other))
        }

        fn gt(&self, other: u8) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), gt_u8(other))
        }

        fn le(&self, other: u8) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), le_u8(other))
        }

        fn ge(&self, other: u8) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), ge_u8(other))
        }
    }

    impl<Entity: OBBlanket> BetweenExt<Entity, u8> for ConditionBuilder<Entity> {
        fn between(&self, this: u8, that: u8) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), between_u8(this, that))
        }
    }

    impl<Entity: OBBlanket> InOutExt<Entity, u8> for ConditionBuilder<Entity> {
        fn member_of(&self, vec: Vec<u8>) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), OneOf_Vec_u8(vec))
        }

        fn not_member_of(&self, vec: Vec<u8>) -> Condition<Entity> {
            Condition::new(self.get_property_attrs(), NotOneOf_u8(vec))
        }
    }
*/
    #[test]
    fn trait_impl_test() {}
}
