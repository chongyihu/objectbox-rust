use std::fs;
use std::path::Path;

use genco::fmt;
use genco::prelude::*;
use genco::quote_in;

use crate::model_json::ModelEntity;
use crate::model_json::ModelInfo;

trait CodeGenEntityExt {
  fn generate_traits(&self) -> Tokens<Rust>;
}

impl CodeGenEntityExt for ModelEntity {
  fn generate_traits(&self) -> Tokens<Rust> {
      let entity = &rust::import("crate", &self.name);
      let schema_id = &rust::import("objectbox::model", "SchemaID");
      let bridge_trait = &rust::import("objectbox::traits", "FBOBBridge");
      let id_trait = &rust::import("objectbox::traits", "IdExt");

      // impl $bridge_trait for $entity {
      //     fn to_fb(self /* TODO, builder: &fb.Builder */) {

      //     }            
      // }

      let (entity_id, _) = crate::parse_colon_separated_integers(&self.id, 0);

      quote! {
        impl $id_trait for $entity {
            fn get_id(&self) -> $schema_id {
              $entity_id
            }
            fn set_id(&mut self, id: $schema_id) {

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
      tokens.append(e.generate_traits());
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

// TODO
  // fn generate_model(&self) -> Tokens<Rust> {
  //     let bridgeTrait = rust::import("objectbox::traits", "FBOBBridge");
  //     let idTrait = rust::import("objectbox::traits", "IdExt");
  //     let tokens: rust::Tokens = quote! {
  //         impl $bridgeTrait for Entity {
  //             fn to_fb(self /* TODO, builder: &fb.Builder */) {

  //             }            
  //         }
          
  //         impl $idTrait for SomeEntity {
  //             fn get_id(&self) -> model::SchemaID {
  //                 1
  //          // partially destructure: ex: let ThreePoint { m1, m2, .. } = p;
  //          // partially destructure: ex: let ThreePoint { m1, m2, .. } = p;
  //          // https://users.rust-lang.org/t/how-can-i-destruct-a-mutable-reference/28967/8
  //             }
  //         // fn set_id(&mut self, id: model::SchemaID) {
  //         // }
  //         }
  //     };
  //     tokens
  // }
}