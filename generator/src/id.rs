// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see quote::format_ident
// TODO replace with serde
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Default, Debug, Clone)]
pub struct IdUid {
  pub id: u64,
  pub uid: u64
}

fn get_uid(rng: &mut ThreadRng) -> u64 {
  rng.gen::<u64>()
}

impl IdUid {
  pub fn zero() -> Self { IdUid{ id: 0, uid: 0 } }

  pub fn to_string(&self) -> String {
    format!("{}:{}", self.id, self.uid)
  }

  fn from_rng_and_previous_id(&mut self, prev_id: u64, rng: &mut ThreadRng) {
    let id = if self.id == 0 { prev_id + 1 } else { self.id };
    let uid = if self.uid == 0 { get_uid(rng) } else { self.uid };
    self.id = id;
    self.uid = uid;
  }
}