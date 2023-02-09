use crate::{c::*, traits::OBBlanket};
use core::marker::PhantomData;
use std::rc::Rc;

// TODO write compile time determined extension blanket traits
// Idea: lock down what which ops are available given the property type
// Do dynamic dispatch down the line, with match.
// Pass enums / tuples down to the builder.

type IdsAndType = Rc<(obx_schema_id, obx_schema_id, u8)>;

#[derive(PartialEq)]
pub(crate) enum ConditionOp /*<T>*/ {
    All,
    Any,

    // IsNull,
    // NotNull,
    Contains(Vec<String>),
    ContainsElement(String),
    StartsWith(String),
    EndsWith(String),

    // TODO Actually type out all the concrete enum parameter types
    // TODO or use a macro
    // Gt(T),
    // GreaterOrEq(T),
    // Lt(T),
    // LessOrEq(T),
    // OneOf(T),
    // NotOneOf(T),

    // TODO remove after writing the macro to generate
    // Eq,
    // NotEq,

    // Between(T, T),

    // Test enums
    TestU8(u8),
}

/// All conditions are collected then passed on to a QueryBuilder
pub struct Condition<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    ids_and_type: Option<IdsAndType>,
    op: ConditionOp,
    group: Option<Vec<Condition<Entity>>>,
}

impl<Entity: OBBlanket> Condition<Entity> {
    fn from_group(new_op: ConditionOp, new_group: Vec<Condition<Entity>>) -> Self {
        let mut new_root = Condition {
            phantom_data: PhantomData,
            ids_and_type: None,
            op: new_op,
            group: None,
        };

        new_root.group = Some(new_group);
        new_root
    }

    fn from_op(other: ConditionOp, ids_and_type: IdsAndType) -> Condition<Entity> {
        Condition {
            phantom_data: PhantomData,
            ids_and_type: Some(ids_and_type),
            op: other,
            group: None,
        }
    }

    pub fn or(self, that: Condition<Entity>) -> Condition<Entity> {
        Self::from_group(ConditionOp::Any, vec![self, that])
    }

    pub fn and(self, that: Condition<Entity>) -> Condition<Entity> {
        Self::from_group(ConditionOp::All, vec![self, that])
    }

    pub fn or_any(self, mut those: Vec<Condition<Entity>>) -> Condition<Entity> {
        those.insert(0, self);
        Self::from_group(ConditionOp::All, those)
    }

    pub fn and_all(self, mut those: Vec<Condition<Entity>>) -> Condition<Entity> {
        those.insert(0, self);
        Self::from_group(ConditionOp::All, those)
    }
}

pub struct ConditionBuilder<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    // entity_id: obx_schema_id, property_id: obx_schema_id, property_type: u8,
    ids_and_type: IdsAndType,
}

impl<Entity: OBBlanket> ConditionBuilder<Entity> {
    fn get_parameters(&self) -> IdsAndType {
        self.ids_and_type.clone()
    }
}

/*
// Note: custom null trait (NullExt)
is_null
not_null
*/
// TODO put this directly into Condition, when values can be `Optional`
// pub trait NullExt<Entity: OBBlanket> {
//     fn is_null() -> Condition<Entity>;
//     fn is_not_null() -> Condition<Entity>;
// }

// TODO figure out if std::ops really doesn't contain <, >, <=, >=
// If op overloading has to be thru, the std::cmp::Partial{Ord,Eq}
// then no op overloading, Because every op return type is bool.
pub trait PartialEq<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: Rhs) -> Condition<Entity>;
    fn ne(&self, other: Rhs) -> Condition<Entity>;
}

pub trait PartialOrd<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn lt(&self, other: Rhs) -> Condition<Entity>;
    fn gt(&self, other: Rhs) -> Condition<Entity>;
    fn le(&self, other: Rhs) -> Condition<Entity>;
    fn ge(&self, other: Rhs) -> Condition<Entity>;
}

/*
// Note: PartialOrd, PartialEq, custom StringExt trait apply
equals_string
not_equals_string
contains_string // custom
contains_element_string // custom
contains_key_value_string // wtf?
starts_with_string // custom
ends_with_string // custom
greater_than_string
greater_or_equal_string
less_than_string
less_or_equal_string
in_strings // custom
any_equals_string // custom
*/
pub trait StringExt<Entity: OBBlanket> {
    fn contains(s: &str) -> Condition<Entity>;
    fn contains_element(s: &str) -> Condition<Entity>;
    // contains_key_value_string // huh?
    fn starts_with(s: &str) -> Condition<Entity>;
    fn ends_with(s: &str) -> Condition<Entity>;
    // fn in_strings(&[&str]) -> Condition; // not sure about the name
    fn any_equals(list: &[&str]) -> Condition<Entity>; // not sure about the input type
}

// TODO blanket later
// pub trait StringBlanket<T: OBBlanket>: StringExt<T> + PartialOrd + PartialEq {}
// impl<T: OBBlanket> StringBlanket<T> for T
// where
//     T: StringExt<T> + PartialOrd + PartialEq,
// {}

// TODO blanket later
// impl<T: OBBlanket> StringBlanket<T> for Condition<T>{}

/*
// Note: PartialOrd and PartialEq apply
equals_int
not_equals_int
greater_than_int
greater_or_equal_int
less_than_int
less_or_equal_int
between_2ints // custom between trait
*/
trait BetweenExt<Entity: OBBlanket, SurroundType>
where
    SurroundType: ?Sized,
{
    fn between(&self, this: SurroundType, that: SurroundType) -> Condition<Entity>;
}

/*
// Note: custom in / not_in trait
in_int64s
not_in_int64s
*/
// in:reserved keyword
trait InOutExt<Entity: OBBlanket, U>
where
    U: Sized,
{
    fn member_of(&self, element: &Vec<U>) -> Condition<Entity>;
    fn not_member_of(&self, element: &Vec<U>) -> Condition<Entity>;
}

/*
// Note: custom in / not_in trait
in_int32s
not_in_int32s

// Note: Only PartialOrd applies here
greater_than_double
greater_or_equal_double
less_than_double
less_or_equal_double
between_2doubles // custom between trait, don't implement like dart?

// Both PartialEq and PartialOrd apply
equals_bytes
greater_than_bytes
greater_or_equal_bytes
less_than_bytes
less_or_equal_bytes
*/

/*
all
any
*/

// TODO
// trait SetExt<Entity: OBBlanket> {
//   or()
//   and()
// }

// TODO order

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::traits::ConditionOp::TestU8;

    impl<Entity: OBBlanket> PartialEq<Entity, u8> for ConditionBuilder<Entity> {
        fn eq(&self, other: u8) -> Condition<Entity> {
            Condition::from_op(TestU8(other), self.get_parameters())
        }

        fn ne(&self, other: u8) -> Condition<Entity> {
            Condition::from_op(TestU8(other), self.get_parameters())
        }
    }

    #[test]
    fn trait_impl_test() {}
}
