use std::marker::PhantomData;
use std::rc::Rc;

use crate::query::enums::ConditionOp;
use crate::traits::OBBlanket;
use crate::c::obx_schema_id;

pub type IdsAndType = Rc<(obx_schema_id, obx_schema_id, u8)>;
/// All conditions are collected then passed on to a QueryBuilder
pub struct Condition<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    ids_and_type: Option<IdsAndType>,
    op: ConditionOp,
    // This could have been a parameter the All/Any enum type
    // but it introduced syntactical noise due to generics
    // to the other enum values. Now we have a (directional) tree.
    group: Option<Vec<Self>>,
}

impl<Entity: OBBlanket> Condition<Entity> {
    pub(crate) fn new_group(op: ConditionOp, group: Vec<Self>) -> Self {
        Self {
            phantom_data: PhantomData,
            op,
            group: Some(group),
            ids_and_type: None::<IdsAndType>,
        }
    }

    pub(crate) fn new(ids_and_type: IdsAndType, op: ConditionOp) -> Self {
        Self {
            phantom_data: PhantomData,
            ids_and_type: Some(ids_and_type),
            op,
            group: None,
        }
    }

    pub fn or(self, that: Condition<Entity>) -> Self {
        Self::new_group(ConditionOp::Any, vec![self, that])
    }

    pub fn and(self, that: Condition<Entity>) -> Self {
        Self::new_group(ConditionOp::All, vec![self, that])
    }

    pub fn or_any(self, mut those: Vec<Condition<Entity>>) -> Self {
        those.insert(0, self);
        Self::new_group(ConditionOp::Any, those)
    }

    pub fn and_all(self, mut those: Vec<Condition<Entity>>) -> Self {
        those.insert(0, self);
        Self::new_group(ConditionOp::All, those)
    }
}
