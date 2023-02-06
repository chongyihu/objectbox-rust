// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// note: literals can be typed: 1_u8
// TODO see quote::format_ident

#[derive(Default, Debug, Clone)]
pub struct IdUid {
    pub id: u64,
    pub uid: u64,
}

impl IdUid {
    pub fn zero() -> Self {
        IdUid { id: 0, uid: 0 }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.id, self.uid)
    }
}
