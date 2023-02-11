
pub(crate) enum ConditionOp {


  Contains(Vec<String>),
  ContainsElement(String),
  ContainsKeyValue(String, String),
  StartsWith(String),
  EndsWith(String),
  AnyEquals(String),

  CaseSensitive(bool),

  All,
  Any,

  IsNull,
  NotNull,

  OrderFlags, // u32

  NoOp,
}
