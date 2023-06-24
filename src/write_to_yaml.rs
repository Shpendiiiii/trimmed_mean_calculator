use std::collections::HashMap;
use serde::Serialize;
use serde_yaml;
use std::io::prelude::*;
use std::time::SystemTime;
use std::fs::File;
use std::io::Error;

#[derive(Serialize)]
struct SimpleKeyVal{
    key: String,
    val: Vec<f64>,
}

struct SimpleKeyValF64{
    key: String,
    val: f64,
}

pub fn insert_vector(desc: &str, vec: &Vec<f64>, file: &mut Result<File, Error>){
    let vals = SimpleKeyVal{
        key: desc.parse().unwrap(),
        val: vec.to_vec(),
    };

    let mut data:HashMap<String, Vec<f64>> = HashMap::new();

    data.insert(vals.key.clone(), vals.val.clone());

    let yaml = serde_yaml::to_string(&data).unwrap();


    file.as_ref().expect("Failed to open file")
        .write_all(yaml.as_bytes())
        .expect("Failed to write to YAML");
}

pub fn insert_f64(desc: &str, result: f64, file: &mut Result<File, Error>){
    let vals = SimpleKeyValF64{
        key: desc.parse().unwrap(),
        val: result,
    };

    let mut data:HashMap<String, f64> = HashMap::new();

    data.insert(vals.key.clone(), vals.val.clone());

    let yaml = serde_yaml::to_string(&data).unwrap();


    file.as_ref().expect("Failed to open file")
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

pub fn create_file() -> Result<File, Error> {
    let file_path = generate_unique_id();
    let file = File::create(&file_path);
    file
}