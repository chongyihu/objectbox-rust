/// TODO implement Drop on QueryProperty etc.
/// TODO whichever holds a C ptr, that has a free/close C fn
/// 
/// Reminder:
/// Expression in dart: box.query(i.greaterThan(0)).build().property(pq);
/// box.query -> QueryBuilder
/// i -> QueryProperty (QP)
/// i.greaterThan(0) -> Condition
/// ..build() -> Query
/// ..property(j) -> PropertyQuery (PQ confusing as hell, I named it, it's my fault)
/// j -> QP (like i)
/// 
/// Traits to reuse: https://doc.rust-lang.org/std/ops/index.html
/// Ops:
/*
enum _ConditionOp {
  isNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  notNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  eq, // std::cmp::Eq
  notEq, // std::ops::Not
  contains,
  containsElement,
  startsWith,
  endsWith,
  gt, // std::cmp::PartialOrd
  greaterOrEq, // std::cmp::PartialOrd
  lt, // std::cmp::PartialOrd
  lessOrEq, // std::cmp::PartialOrd
  oneOf,
  notOneOf,
  between,
}
*/