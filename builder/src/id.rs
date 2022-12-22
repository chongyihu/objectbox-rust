// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see quote::format_ident
// TODO replace with serde

#[derive(Default, Debug, Clone)]
pub struct IdUid {
  pub id: u64,
  pub uid: u64
}

impl IdUid {
  pub fn to_string(&self) -> String {
    format!("{}:{}", self.id, self.uid)
  }
}