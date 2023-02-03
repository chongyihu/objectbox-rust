pub mod id;
pub mod model_json;
pub mod ob_consts;

use std::{path::PathBuf, fs};
use glob::glob;
use model_json::{ModelEntity, ModelInfo};
use rand::Rng;
use rand;

// TODO implement collision detection and evasion with predefined id and uid
// TODO general idea: maintain a set of id and uid, when incrementing the counter,
// TODO check for collision, if yes, increment/generate again, if no, assign value
fn parse_colon_separated_integers(str: &String, counter: u64) -> (u64, u64) {
    use substring::Substring;
    let mut id: u64 = 0;
    let mut uid: u64 = 0;
    let mut rng = rand::thread_rng();
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
    fn assign_id_to_indexables(&mut self) -> &mut Self;
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
        self
    }

    fn assign_id_to_indexables(&mut self) -> &mut Self {
        let mut counter: u64 = 1;
        let mut rng = rand::thread_rng();
        for e in self.as_mut_slice() {
            for p in e.properties.as_mut_slice() {
                if p.index_id.is_some() {
                    p.index_id = Some(format!("{}:{}", counter, rng.gen::<u64>()));
                    counter+=1;
                }
            }
        }
        self
    }
}

mod code_gen;
use code_gen::CodeGenExt;

pub fn generate_assets(out_path: &PathBuf, cargo_manifest_dir: &PathBuf) {
    // Read <entity>.objectbox.info and consolidate into
    let pbs = glob_generated_json(out_path);

    if pbs.is_empty() {
        println!("cargo:warning=No entities declared!");
        return
    }

    let json_dest_path = &cargo_manifest_dir.as_path().join("src/objectbox-model.json");
    let ob_dest_path = &cargo_manifest_dir.as_path().join("src/objectbox_gen.rs");
    if let Ok(exists) = json_dest_path.try_exists() {
        // Exports everything a user needs from objectbox, fully generated
        if exists {
            let model_info_from_one_file = ModelInfo::from_json_file(json_dest_path);

            if model_info_from_one_file.entities.len() != pbs.len() {
                println!("cargo:warning=The number of entities have changed,\nconsider backing up and/or modifying or deleting objectbox-model.json");
            }        

            model_info_from_one_file.generate_code(ob_dest_path);
            return;
        }
    }

    let mut entities = Vec::<ModelEntity>::new();

    // read what is provided by the user
    entities
        .add_entities_to_model(pbs.as_slice())
        .assign_id_to_entities().assign_id_to_indexables();

    ModelInfo::from_entities(entities.as_slice())
    .write_json(json_dest_path)
    .generate_code(ob_dest_path);
}