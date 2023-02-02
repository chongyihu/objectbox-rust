// TODO implement Drop on QueryProperty etc.
// TODO whichever holds a C ptr, that has a free/close C fn
// 
// Reminder:
// Expression in dart: box.query(i.greaterThan(0)).build().property(pq);
// box.query -> QueryBuilder
// i -> QueryProperty (QP)
// i.greaterThan(0) -> Condition
// ..build() -> Query
// ..property(j) -> PropertyQuery (PQ) PQ vs QP are confusing as hell, I named it, mea culpa
// j -> QP (like i)
// 
// Traits to reuse: https://doc.rust-lang.org/std/ops/index.html
// Ops: https://doc.rust-lang.org/book/appendix-02-operators.html
/*
enum _ConditionOp {
  isNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  notNull, // TODO only feasible when Option<OB_Rust_Primitive> is introduced
  eq, // std::cmp::PartialEq, eq
  notEq, // std::ops::PartialEq, ne
  contains,
  containsElement,
  startsWith,
  endsWith,
  gt, // std::cmp::PartialOrd, gt
  greaterOrEq, // std::cmp::PartialOrd, ge
  lt, // std::cmp::PartialOrd, lt
  lessOrEq, // std::cmp::PartialOrd, le
  oneOf,
  notOneOf,
  between,
}

// TODO even better, check predicates: https://docs.rs/predicates/2.1.5/predicates/index.html
// e.g. box.query(qp)

// For lack of variadic args on .query(), use query(vec!(condition...));
*/