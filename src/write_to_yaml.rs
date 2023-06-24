use std::collections::HashMap;
use serde::Serialize;
use serde_yaml;
use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::Rng;
use std::time::SystemTime;
use std::fs::File;

#[derive(Serialize)]
struct SimpleKeyVal{
    key: String,
    val: Vec<f64>
}

pub fn insert_vector(vec: &Vec<f64>){
    let vals = SimpleKeyVal{
        key: format!("input"),
        val: vec.to_vec(),
    };

    let mut data:HashMap<String, Vec<f64>> = HashMap::new();

    data.insert(vals.key.clone(), vals.val.clone());

    let yaml = serde_yaml::to_string(&data).unwrap();

    
    let file = File::create(generate_unique_id());

    file.expect("Failed to open file")
        .write_all(yaml.as_bytes())
        .expect("Failed to write to YAML");
}

fn generate_unique_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Error")
        .as_secs();
    format!("output_{}.yaml", timestamp)
}