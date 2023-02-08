use core::marker::PhantomData;
use crate::{c::*, traits::OBBlanket};

// TODO write compile time determined extension blanket traits
// Idea: lock down what which ops are available given the property type
// Do dynamic dispatch down the line, with match.
// Pass enums / tuples down to the builder.

// Don't do any unnecessary up/down casting, this is a pita.
// Get ready to kill your darlings.

/*
// Note: custom null trait (NullExt)
or
and
*/

/// All condition are collected then passed on to a QueryBuilder
pub struct Condition<Entity: OBBlanket> {
  phantom_data: PhantomData<Entity>
}

// impl Condition {
//   pub fn or(&self, that: &Condition) -> AnyCondition {

//   }

//   pub fn and(&self, that: &Condition) -> AllCondition {

//   }
//   pub fn or_many(&self, those: &[Condition]) -> AnyCondition {

//   }

//   pub fn and_many(&self, those: &[Condition]) -> AllCondition {

//   }
// }

struct ConditionBuilder<Entity: OBBlanket> {
  phantom_data: PhantomData<Entity>,
  entity_id: obx_schema_id,
  property_id: obx_schema_id,
}

/*
// Note: custom null trait (NullExt)
is_null
not_null
*/
trait NullExt<Entity: OBBlanket> {
  fn is_null() -> Condition<Entity>;
  fn is_not_null() -> Condition<Entity>;
}

// TODO figure out if std::ops really doesn't contain <, >, <=, >=
// If op overloading has to be thru, the std::cmp::Partial{Ord,Eq}
// then no op overloading, Because every op return type is bool.
pub trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> Rhs;
    fn ne(&self, other: &Rhs) -> Rhs;
}

pub trait PartialOrd<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn lt(&self, other: &Rhs) -> Rhs;
    fn gt(&self, other: &Rhs) -> Rhs;
    fn le(&self, other: &Rhs) -> Rhs;
    fn ge(&self, other: &Rhs) -> Rhs;
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
pub trait StringBlanket<T: OBBlanket>: StringExt<T> + PartialOrd + PartialEq {}
impl<T: OBBlanket> StringBlanket<T> for T where T: StringExt<T> + PartialOrd + PartialEq {
  // TODO
}
pub struct StringCondition<Entity: OBBlanket>{
  phantom_data: PhantomData<Entity>,
}
// TODO
// impl<Entity: OBBlanket> StringBlanket<Entity> for StringCondition<Entity> {

// }


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
trait BetweenExt<Entity: OBBlanket, U> {
  fn between(this: U, that: Entity) -> Condition<Entity>;
}

/*
// Note: custom in / not_in trait
in_int64s
not_in_int64s
*/
// in:reserved keyword
trait InOutExt<Entity: OBBlanket, U> {
  fn member_of(element: &[U]) -> Condition<Entity>;
  fn not_member_of(element: &[U]) -> Condition<Entity>;
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