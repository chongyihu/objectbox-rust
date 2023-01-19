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

fn tokens_to_string(tokens: &Tokens<Rust>) -> Vec<u8> {
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

  w.into_inner()
}

trait CodeGenEntityExt {
  fn get_id_property(&self) -> Option<&ModelProperty>;
  fn generate_id_trait(&self) -> Tokens<Rust>;
  fn generate_fb_trait(&self) -> Tokens<Rust>;
}

// fn from_u32(n: u32) -> Option<char> {
//   std::char::from_u32(n)
// }

// fn to_u32(c: char) -> u32 {
//   c as u32
// }

fn encode_to_fb(field_type: u32, i: usize, name: &String) -> Tokens<Rust> {
  let wip_offset = &rust::import("flatbuffers", "WIPOffset");
  let new_tokens: Tokens<Rust> = match field_type {
    ob_consts::OBXPropertyType_StringVector => {
      quote! {
        let strs_vec_$i = self.$name.iter()
        .map(|s|builder.create_string(s.as_str()))
        .collect::<Vec<$wip_offset<&str>>>();
        let vec_$i = builder.create_vector(strs_vec_$i.as_slice());
        builder.push_slot_always($i, vec_$i);
      }
    },
    ob_consts::OBXPropertyType_ByteVector => {
      quote! {
        let byte_vec_$i = builder.create_vector(&self.$name.as_slice());
        builder.push_slot_always($i, byte_vec_$i);
      }
    },
    ob_consts::OBXPropertyType_String => {
      quote! {
        let str_$i = builder.create_string(self.$name.as_str());
        builder.push_slot_always($i, str_$i);
      }
    },
    ob_consts::OBXPropertyType_Char => {
      // TODO test endianness
      quote! {
        builder.push_slot_always($i, self.$name as u32);
      }
    },
    _ => {
      quote! {
        builder.push_slot_always($i, self.$name);
      }
    }
  };
  new_tokens
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

  // TODO Factory<>, FactoryHelper<>, map.insert...boxed factory as factory helper
  fn generate_fb_trait(&self) -> Tokens<Rust> {
    let entity = &rust::import("crate", &self.name);
    let bridge_trait = &rust::import("objectbox::traits", "FBOBBridge");
    let flatbuffer_builder = &rust::import("objectbox::flatbuffers", "FlatBufferBuilder");

    let props: Vec<Tokens<Rust>> = self.properties.iter().enumerate()
    .map(|(i, p)| encode_to_fb(p.type_field, i, &p.name) ).collect();
    
    quote! {
      impl $bridge_trait for $entity {
        fn to_fb(self, builder: &mut $flatbuffer_builder) {
          let wip_offset_unfinished = builder.start_table();
          $props
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
    for e in self.entities.iter() {
        tokens.append(e.generate_id_trait());
        tokens.append(e.generate_fb_trait());
    }

    let vector = tokens_to_string(tokens);

    let utf = match std::str::from_utf8(vector.as_slice()) {
      Ok(utf) => utf,
      Err(error) => panic!("There is a problem with converting bytes to utf8: {}", error)
    };

    let syntax_tree = match syn::parse_file(utf) {
      Ok(parsed) => parsed,
      Err(error) => panic!("There is a problem with parsing the generated rust code: {}", error)
    };

    // it seems that genco's code formatting is broken on stable
    let formatted = prettyplease::unparse(&syntax_tree);

    if let Err(error) = fs::write(&path, formatted.as_str()) {
        panic!("There is a problem writing the generated rust code: {:?}", error);
    }
  }
}

