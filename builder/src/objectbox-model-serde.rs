use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

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
    pub id: String,
    pub last_property_id: String,
    pub name: String,
    pub properties: Vec<Property>,
    pub relations: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub flags: Option<i64>,
}
