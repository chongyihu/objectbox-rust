use std::{path::PathBuf, fs};
use glob::glob;
use rand::Rng;
extern crate rand;

#[path = "./model_json.rs"]
pub mod model_json;

#[path = "./id.rs"]
pub mod id;

fn parse_comma_separated_integers(some_id: &String, counter: u64) -> (u64, u64) {
    use substring::Substring;
    let mut id: u64 = 0;
    let mut uid: u64 = 0;
    if !some_id.is_empty() {
        if some_id.starts_with(":") {
            let right = some_id.substring(1, some_id.len());
            if let Ok(u) = str::parse::<u64>(right) {
                uid = u;
            }else {
                panic!("A u64 could not be parsed from this string");
            }
        }else if some_id.ends_with(":") {
            let left = some_id.substring(0, some_id.len()-1);
            if let Ok(u) = str::parse::<u64>(left) {
                id = u;
            }else {
                panic!("A u64 could not be parsed from this string");
            }
        }else {
            let collect: Vec<&str> = some_id.split(":").collect();
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

pub fn generate_assets(out_path: &PathBuf, cargo_manifest_dir: &PathBuf) {
    // Read <entity>.objectbox.info and consolidate into
    let pbs = glob_generated_json(out_path);

    // read what is provided by the user
    let mut entities = Vec::<model_json::Entity>::new();
    for pb in pbs.iter() {
        let path = pb.as_path();
        let string = fs::read_to_string(path).expect("Unable to read file");
        if let Ok(r) = serde_json::from_str::<model_json::Entity>(&string) {
            entities.push(r);
        }
    }

    // fill in the gaps
    let mut counter: u64 = 0;
    for e in entities.iter_mut() {
        let id = parse_comma_separated_integers(&e.id, counter);
        counter = id.0;
        e.id = format!("{}:{}", id.0, id.1);

        let mut counter_p: u64 = 0;
        for v in e.properties.iter_mut() {
            let id = parse_comma_separated_integers(&v.id, counter_p);
            counter_p = id.0;
            v.id = format!("{}:{}", id.0, id.1);
        }

        let last_property = e.properties.last().unwrap();
        e.last_property_id = last_property.id.clone();
    }

    if entities.len() != 0 {
        model_json::Root::from_entities(entities).write(&cargo_manifest_dir);
    }
    
    // objectbox-generated.rs
    // TODO now write the rest of the code
}