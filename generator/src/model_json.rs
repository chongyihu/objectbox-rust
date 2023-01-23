use genco::prelude::Rust;
use genco::prelude::rust;
use genco::tokens::quoted;
use genco::quote;
use genco::Tokens;
use serde_derive::{Deserialize,Serialize};
use serde_json::Value;

use std::env;
use std::fs;
use std::path::{PathBuf, Path};

use crate::ob_consts;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    #[serde(rename = "_note1")]
    pub note1: String,
    #[serde(rename = "_note2")]
    pub note2: String,
    #[serde(rename = "_note3")]
    pub note3: String,
    pub entities: Vec<ModelEntity>,
    pub last_entity_id: String,
    pub last_index_id: String,
    pub last_relation_id: String,
    pub last_sequence_id: String,
    pub model_version: u64,
    pub model_version_parser_minimum: u64,
    pub retired_entity_uids: Vec<u64>,
    pub retired_index_uids: Vec<u64>,
    pub retired_property_uids: Vec<u64>,
    pub retired_relation_uids: Vec<u64>,
    pub version: u64,
}

impl ModelInfo {
    pub fn from_entities(entities: &[ModelEntity]) -> Self {
      let last_entity = entities.last().unwrap(); // TODO remove unwrap, unpack result and return proper error
      let last_entity_id = last_entity.id.as_str();
      ModelInfo {
        note1: String::from("KEEP THIS FILE! Check it into a version control system (VCS) like git."),
        note2: String::from("ObjectBox manages crucial IDs for your object model. See docs for details."),
        note3: String::from("If you have VCS merge conflicts, you must resolve them according to ObjectBox docs."),
        entities: entities.to_vec(), // rehydrate from slice to vec for JSON des, all of this without cloning
        last_entity_id: last_entity_id.to_string(),
        last_index_id: String::from(""), // TODO
        last_relation_id: String::from(""), // TODO
        last_sequence_id: String::from(""), // TODO
        model_version: 5,
        model_version_parser_minimum: 5,
        retired_entity_uids: Vec::new(), // TODO
        retired_index_uids: Vec::new(), // TODO
        retired_property_uids: Vec::new(), // TODO
        retired_relation_uids: Vec::new(), // TODO
        version: 1,
      }
    }

    pub fn write_json(&mut self, dest_path: &PathBuf) -> &mut Self {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            match fs::write(&dest_path, json) {
                Err(error) => panic!("Problem writing the objectbox-model.json file: {:?}", error),
                _ => {},
            }
        }
        self
    }

    pub fn from_json_file(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str(content.as_str()) {
                    Ok(json) => return json,
                    Err(error) => panic!("Problem parsing the json: {:?}", error),
                }
            }
            Err(error) => panic!("Problem reading the json file: {:?}", error),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelEntity {
    pub id: String, // iduid = "1:12341820347123498124"
    pub last_property_id: String,
    pub name: String,
    pub properties: Vec<ModelProperty>,
    pub relations: Vec<Value>, // TODO
}

impl ModelEntity {
    pub fn write(&mut self) {
        if let Some(out_dir) = env::var_os("OUT_DIR") {
            let dest_path = Path::new(&out_dir).join(format!("{}.objectbox.info", self.name.clone()));
            if let Ok(json) = serde_json::to_string(self) {
                let result = fs::write(
                    &dest_path,
                    json.as_str(),
                    );
                match result {
                    Err(error) => panic!("{}", error),
                    _ => {}
                }
            }
        }else {
            panic!("Missing OUT_DIR environment variable, due to calling this function outside of build.rs");
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelProperty {
    pub id: String, // iduid = "1:12341820347123498124"
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: ob_consts::OBXPropertyType,
    #[serde(skip_serializing_if="Option::is_none")]
    pub flags: Option<ob_consts::OBXPropertyFlags>,
}

fn split_id(input: &str) -> (&str, &str) {
    let v: Vec<&str> = input.split(':').collect();
    (v[0], v[1])
}

impl ModelProperty {
    pub fn as_fluent_builder_invocation(&self) -> Tokens<Rust> {
        let flags = if let Some(f) = self.flags { f } else { 0 };
        let (id, uid) = split_id(&self.id);

        quote! {
            .property(
                $(quoted(self.name.as_str())),
                $id, $uid,
                $(self.type_field),
                $flags
            )
        }
    }

    pub fn as_struct_property_default(&self) -> Tokens<Rust> {
        let name = &self.name;
        match self.type_field {
            ob_consts::OBXPropertyType_StringVector => quote! {
                $name: Vec::<String>::new()
            },
            ob_consts::OBXPropertyType_ByteVector => quote! {
                $name: Vec::<u8>::new()
            },
            ob_consts::OBXPropertyType_String => quote! {
                $name: String::from("")
            },
            ob_consts::OBXPropertyType_Char => quote! {
                $name: char::from(0)
            },
            ob_consts::OBXPropertyType_Bool => quote! {
                $name: false
            },
            ob_consts::OBXPropertyType_Float => quote! {
                $name: 0.0
            },
            ob_consts::OBXPropertyType_Double => quote! {
                $name: 0.0
            },
            // rest of the integer types
            _ => quote! {
                $name: 0
            },
        }
    }

    pub fn as_assigned_property(&self) -> Tokens<Rust> {
        let fuo = &rust::import("objectbox::flatbuffers", "ForwardsUOffset");
        let fvec = &rust::import("objectbox::flatbuffers", "Vector");
        let iduid_id = split_id(self.id.as_str()).0;

        let name = &self.name;
        match self.type_field {
            ob_consts::OBXPropertyType_StringVector => quote! {
                let fb_vec_$name = table.get::<$fuo<$fvec<$fuo<&str>>>>($iduid_id, None);
                if let Some(sv) = fb_vec_$name {
                    *$name = sv.iter().map(|s|s.to_string()).collect();
                }
            },
            ob_consts::OBXPropertyType_ByteVector => quote! {
                let fb_vec_$name = table.get::<$fuo<$fvec<u8>>>($iduid_id, None);
                if let Some(bv) = fb_vec_$name {
                    *$name = bv.bytes().to_vec();
                }
            },
            // TODO research clear the buffer, and read the slice instead
            // TODO see what's faster
            ob_consts::OBXPropertyType_String => quote! {
                if let Some(s) = table.get::<$fuo<&str>>($iduid_id, None) {
                    *$name = s.to_string();
                }
            },
            // TODO will this work with objectbox? rust char = 4x u8 = 32 bits
            // TODO write test for this specifically
            ob_consts::OBXPropertyType_Char => quote! {
                let $(name)_u32 = table.get::<u32>($iduid_id, Some(0)).unwrap();
                if let Some(c) = std::char::from_u32($(name)_u32) {
                    *$name = c;
                }
            },
            ob_consts::OBXPropertyType_Bool => quote! {
                *$name = table.get::<bool>($iduid_id, Some(false)).unwrap();
            },
            ob_consts::OBXPropertyType_Float => quote! {
                *$name = table.get::<f32>($iduid_id, Some(0.0)).unwrap();
            },
            ob_consts::OBXPropertyType_Double => quote! {
                *$name = table.get::<f64>($iduid_id, Some(0.0)).unwrap();
            },
            // rest of the integer types
            _ => {
                let unsigned_flag = match self.flags {
                    Some(f) => f,
                    _ => 0
                };
                let sign: Tokens<Rust> = if (unsigned_flag & ob_consts::OBXPropertyFlags_UNSIGNED) == ob_consts::OBXPropertyFlags_UNSIGNED
                { quote!(u) } else { quote!(i) };
    
                let bits: Tokens<Rust> = match self.type_field {
                    ob_consts::OBXPropertyType_Byte => quote!(8),
                    ob_consts::OBXPropertyType_Short => quote!(16),
                    ob_consts::OBXPropertyType_Int => quote!(32),
                    ob_consts::OBXPropertyType_Long => quote!(64),
                    _ => panic!("Unknown OBXPropertyType")
                };
                quote! {
                    *$name = table.get::<$sign$bits>($iduid_id, Some(0)).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn model_property_fluent_builder_test() {
    let Ok(str) = ModelProperty {
        id: "1:2".to_owned(),
        name: "name".to_owned(),
        type_field: 0,
        flags: Some(0)
    }.as_fluent_builder_invocation().to_string();
    assert_eq!(str, ".property(name, 0, 0, 1, 2)");
}