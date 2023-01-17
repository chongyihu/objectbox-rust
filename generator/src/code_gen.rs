use core::panic;
use std::fs;
use std::path::Path;

use genco::fmt;
use genco::prelude::*;

use crate::model_json::ModelEntity;
use crate::model_json::ModelInfo;
use crate::model_json::ModelProperty;
use crate::ob_consts;

// use flatbuffers::FlatBufferBuilder;

trait CodeGenEntityExt {
  fn get_id_property(&self) -> Option<&ModelProperty>;
  fn generate_id_trait(&self) -> Tokens<Rust>;
  fn generate_fb_trait(&self) -> Tokens<Rust>;
}

fn decide_fb_encoding(field_type: u32, i: usize, name: &String) -> String {
  match field_type {
    ob_consts::OBXPropertyType_ByteVector => {
      // format!("builder.push_slot_always({}, builder.create_vector({}));", i, name)
      format!("// not available atm, because Vec<String>, char, Vec<u64>, could be all stored the same way")
    },
    ob_consts::OBXPropertyType_String => {
      format!("let str = builder.create_string(self.{}.as_str());\nbuilder.push_slot_always({}, str);", name, i)
    },
    _ => {
      format!("builder.push_slot_always({}, self.{});", i, name)
    }
  }
}

impl CodeGenEntityExt for ModelEntity {
  // TODO throw error during macro parsing
  // TODO if no ID, or multiple are defined
  fn get_id_property(&self) -> Option<&ModelProperty> {
    for p in self.properties.iter() {
      if let Some(flags) = p.flags {
        if flags & 1 == 1 {
          return Some(&p);
        }
      }
    }
    None
  }

  fn generate_id_trait(&self) -> Tokens<Rust> {
      let entity = &rust::import("crate", &self.name);
      let schema_id = &rust::import("objectbox::model", "SchemaID");
      let id_trait = &rust::import("objectbox::traits", "IdExt");

      let id = self.get_id_property();

      let p = if let Some(p) = id {
        p
      }else {
        panic!("No ID was defined for {}", self.name);
      };

      // TODO char = 4x bytes = Vec<u8>... as_slice()
      // TODO Vec<String>, vec anything... as_slice()
      // TODO Factory<>, FactoryHelper<>, map.insert...boxed factory as factory helper
      quote! {
        impl $id_trait for $entity {
          fn get_id(&self) -> $schema_id {
            self.$(p.name.as_str())
          }
          fn set_id(&mut self, id: $schema_id) {
            self.$(p.name.as_str()) = id;
          }
        }
      }
  }

  fn generate_fb_trait(&self) -> Tokens<Rust> {
    let entity = &rust::import("crate", &self.name);
    let bridge_trait = &rust::import("objectbox::traits", "FBOBBridge");
    let flatbuffer_builder = &rust::import("objectbox::flatbuffers", "FlatBufferBuilder");

    // Caveat! When decoding/encoding flatbuffers note that
    // C's char is 1 byte, Rust's is 4 bytes (aka a vector, n=4 bytes)
    let builder_props: Vec<String> = self.properties.iter().enumerate().map(|(i, p)| decide_fb_encoding(p.type_field, i, &p.name) ).collect();
    let props = builder_props.join("\n");
    
    quote! {
      impl $bridge_trait for $entity {
        fn to_fb(self, builder: &mut $flatbuffer_builder) {
          let wip_offset_unfinished = builder.start_table();
          $(props.as_str())
          let wip_offset_finished = builder.end_table(wip_offset_unfinished);
          builder.finish_minimal(wip_offset_finished);
        }
      }
    }
  }
}

// TODO Fix visibility on all the trait extensions
pub(crate) trait CodeGenExt {
  // fn generate_model(&self) -> Tokens<Rust>; // TODO
  fn generate_code(&self, path: &Path);
}

impl CodeGenExt for ModelInfo {
  fn generate_code(&self, path: &Path) {
    let tokens = &mut rust::Tokens::new();
    for (i, e) in self.entities.iter().enumerate() {
        tokens.append(e.generate_id_trait());
        tokens.append(e.generate_fb_trait());
    }

    // Vec<u8> implements std::io::Write
    let mut w = fmt::IoWriter::new(Vec::<u8>::new());

    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default()
    // Prettier imports and use.
    .with_default_import(rust::ImportMode::Qualified);

    // TODO test assumption: I suspect indentation is fubar without nightly
    if let Err(error) = tokens.format_file(&mut w.as_formatter(&fmt), &config) {
      panic!("{:?}", error);
    }

    let vector = w.into_inner();
    let utf_result = std::str::from_utf8(&vector);

    if let Ok(str) = utf_result {
        if let Err(error) = fs::write(&path, str) {
            panic!("Problem writing the objectbox.rs file: {:?}", error);
        }
    }
  }
}

