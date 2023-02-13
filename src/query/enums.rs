#![allow(non_camel_case_types)]

pub(crate) enum ConditionOp {
    Contains(String),
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

    OrderFlags(u32),

    NoOp,

    // Generated
    Eq_i64(i64),
    Ne_i64(i64),
    Lt_i64(i64),
    Gt_i64(i64),
    Le_i64(i64),
    Ge_i64(i64),
    Eq_f64(f64),
    Ne_f64(f64),
    Lt_f64(f64),
    Gt_f64(f64),
    Le_f64(f64),
    Ge_f64(f64),
    Eq_string(String),
    Ne_string(String),
    Lt_string(String),
    Gt_string(String),
    Le_string(String),
    Ge_string(String),
    Eq_vecu8(Vec<u8>),
    // Ne_vecu8(Vec<u8>), // No op
    Lt_vecu8(Vec<u8>),
    Gt_vecu8(Vec<u8>),
    Le_vecu8(Vec<u8>),
    Ge_vecu8(Vec<u8>),
    Eq_vecstring(Vec<String>),
    Ne_vecstring(Vec<String>),
    Lt_vecstring(Vec<String>),
    Gt_vecstring(Vec<String>),
    Le_vecstring(Vec<String>),
    Ge_vecstring(Vec<String>),
    Between_i64(i64, i64),
    Between_f64(f64, f64),
    In_i32(Vec<i32>),
    NotIn_i32(Vec<i32>),
    In_i64(Vec<i64>),
    NotIn_i64(Vec<i64>),
    In_String(Vec<String>),
    // NotIn_String(Vec<String>), // No op
}
