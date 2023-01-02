use serde_derive::{Deserialize,Serialize};
use serde_json::Value;

use std::env;
use std::fs;
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "_note1")]
    pub note1: String,
    #[serde(rename = "_note2")]
    pub note2: String,
    #[serde(rename = "_note3")]
    pub note3: String,
    pub entities: Vec<Entity>,
    pub last_entity_id: String,
    pub last_index_id: String,
    pub last_relation_id: String,
    pub last_sequence_id: String,
    pub model_version: i64,
    pub model_version_parser_minimum: i64,
    pub retired_entity_uids: Vec<Value>,
    pub retired_index_uids: Vec<Value>,
    pub retired_property_uids: Vec<Value>,
    pub retired_relation_uids: Vec<Value>,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String, // iduid = "1:12341820347123498124"
    pub last_property_id: String,
    pub name: String,
    pub properties: Vec<Property>,
    pub relations: Vec<Value>, // TODO
    // #[serde(skip_serializing_if="Option::is_none")]
    // pub path: Option<String>,
}

impl Entity {
    // pub fn set_path(&mut self, path: Option<String>) -> &mut Self {
    //     self.path = path;
    //     self
    // }

    pub fn write(&mut self) {
      let out_dir = env::var_os("OUT_DIR").unwrap();
      let dest_path = Path::new(&out_dir).join(format!("{}.objectbox.info", self.name.clone()));
      fs::write(
          &dest_path,
          format!("{}", serde_json::to_string(self).unwrap()),
          ).unwrap();
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: String, // iduid = "1:12341820347123498124"
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: u16,
    #[serde(skip_serializing_if="Option::is_none")]
    pub flags: Option<u16>,
}
