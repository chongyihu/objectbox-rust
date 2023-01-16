use std::{path::{PathBuf, Path}, fs};
use glob::glob;
use model_json::{ModelEntity, ModelInfo};
use rand::Rng;
extern crate rand;

pub mod model_json;
pub mod id;

// TODO implement collision detection and evasion with predefined id and uid
// TODO general idea: maintain a set of id and uid, when incrementing the counter,
// TODO check for collision, if yes, increment/generate again, if no, assign value
fn parse_colon_separated_integers(str: &String, counter: u64) -> (u64, u64) {
    use substring::Substring;
    let mut id: u64 = 0;
    let mut uid: u64 = 0;
    if !str.is_empty() {
        if str.starts_with(":") {
            let right = str.substring(1, str.len());
            if let Ok(u) = str::parse::<u64>(right) {
                uid = u;
            }else {
                panic!("A u64 could not be parsed from this string");
            }
        }else if str.ends_with(":") {
            let left = str.substring(0, str.len()-1);
            if let Ok(u) = str::parse::<u64>(left) {
                id = u;
            }else {
                panic!("A u64 could not be parsed from this string");
            }
        }else {
            let collect: Vec<&str> = str.split(":").collect();
            if collect.len() == 2 {
              let left = collect[0];
              let right = collect[1];
              if let Ok(i) = str::parse::<u64>(left) {
                id = i;
              }else {
                panic!("A u64 could not be parsed from this string");
              }
              if let Ok(u) = str::parse::<u64>(right) {
                uid = u;
              }else {
                panic!("A u64 could not be parsed from this string");
              }
            }
        }
    }
    if id == 0 {
        id = counter + 1;
    }
    if uid == 0 {
        let mut rng = rand::thread_rng();
        uid = rng.gen::<u64>();
    }
    (id, uid)
}

#[cfg(test)]
mod tests {
    use crate::parse_colon_separated_integers;

    #[test]
    fn colon_separated_integers() {
        {
            let result = parse_colon_separated_integers(&String::from("1:1337"), 0);
            assert_eq!(result, (1,1337));
        }
        {
            let result = parse_colon_separated_integers(&String::from(":1337"), 0);
            assert_eq!(result, (1,1337));    
        }
        {
            let result = parse_colon_separated_integers(&String::from("7:"), 0);
            assert_eq!(result.0, 7);
        }
    }
}

fn glob_generated_json(out_path: &PathBuf) -> Vec<PathBuf> {
    let suffix = "*.objectbox.info";
    let glob_path = format!("{}/{}", out_path.to_str().unwrap_or_default(), suffix);
    let expect = format!("Failed to read glob pattern {}", suffix);
    let mut pbs: Vec<PathBuf> = Vec::new();
    for entry in glob(&glob_path).expect(&expect) {
        match entry {
            Ok(path) => {
                pbs.push(path.clone());
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    pbs
}

trait EntityVecHelper {
    fn add_entities_to_model(&mut self, path_buffers: &[PathBuf]) -> &mut Self;
    fn assign_id_to_entities(&mut self) -> &mut Self;
}

impl EntityVecHelper for Vec<ModelEntity> {
    fn add_entities_to_model(&mut self, path_buffers: &[PathBuf]) -> &mut Self {
        for pb in path_buffers.iter() {
            let path = pb.as_path();
            let string = fs::read_to_string(path).expect("Unable to read file");
            if let Ok(r) = serde_json::from_str::<ModelEntity>(&string) {
                self.push(r);
            }
        }
        self
    }

    fn assign_id_to_entities(&mut self) -> &mut Self {
        // TODO harmonize values from existing objectbox-model.json (figure out exact requirements first)
        // fill in the missing id:uids
        let mut counter: u64 = 0;
        for e in self.iter_mut() {
            let id = parse_colon_separated_integers(&e.id, counter);
            counter = id.0;
            e.id = format!("{}:{}", id.0, id.1);
    
            let mut counter_p: u64 = 0;
            for v in e.properties.iter_mut() {
                let id = parse_colon_separated_integers(&v.id, counter_p);
                counter_p = id.0;
                v.id = format!("{}:{}", id.0, id.1);
            }
    
            let last_property = e.properties.last().unwrap();
            e.last_property_id = last_property.id.clone();
        }
        // sort by entity name
        self.sort_by(|a, b| b.name.cmp(&a.name));
        self
    }
}

use genco::fmt;
use genco::prelude::*;
use genco::quote_in;

trait CodeGenEntityExt {
    fn generate_traits(&self) -> Tokens<Rust>;
}

impl CodeGenEntityExt for ModelEntity {
    fn generate_traits(&self) -> Tokens<Rust> {
        let entity = &rust::import("crate", &self.name);
        let schema_id = rust::import("objectbox::model", "SchemaID");
        let bridge_trait = rust::import("objectbox::traits", "FBOBBridge");
        let id_trait = rust::import("objectbox::traits", "IdExt");
        quote! {
            impl $bridge_trait for $entity {
                fn to_fb(self /* TODO, builder: &fb.Builder */) {

                }            
            }
            
            impl $id_trait for $entity {
                fn get_id(&self) -> $schema_id {
                    1
                }
            // fn set_id(&mut self, id: model::SchemaID) {
            // }
            }
        }
    }
}

trait CodeGenExt {
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

    let fmt = fmt::Config::from_lang::<Rust>();
    let config = rust::Config::default();
    // Default format state for Rust.
    let format = rust::Format::default();

    if let Err(error) = tokens.format(&mut w.as_formatter(&fmt), &config, &format) {
        // println!("cargo:warning={:?}", error);
        panic!("{:?}", error);
    }
    let vector = w.into_inner();
    let utf_result = std::str::from_utf8(&vector);

    if let Ok(str) = utf_result {
        match fs::write(&path, str) {
            Err(error) => panic!("Problem writing the objectbox.rs file: {:?}", error),
            Ok(_) => {},
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
    //             }
    //         // fn set_id(&mut self, id: model::SchemaID) {
    //         // }
    //         }
    //     };
    //     tokens
    // }
}

pub fn generate_assets(out_path: &PathBuf, cargo_manifest_dir: &PathBuf) {
    // Read <entity>.objectbox.info and consolidate into
    let pbs = glob_generated_json(out_path);

    if pbs.is_empty() {
        println!("cargo:warning=No entities declared!");
        return
    }

    let json_dest_path = &cargo_manifest_dir.as_path().join("src/objectbox-model.json");
    let ob_dest_path = &cargo_manifest_dir.as_path().join("src/objectbox.rs");
    if let Ok(exists) = json_dest_path.try_exists() {
        // Exports everything a user needs from objectbox, fully generated
        if exists {
            ModelInfo::from_json_file(json_dest_path).generate_code(ob_dest_path);
            return;    
        }
    }

    let mut entities = Vec::<ModelEntity>::new();

    // read what is provided by the user
    entities
        .add_entities_to_model(pbs.as_slice())
        .assign_id_to_entities();

    ModelInfo::from_entities(entities.as_slice())
    .write_json(json_dest_path)
    .generate_code(ob_dest_path);

}