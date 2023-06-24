use std::collections::HashMap;
use serde::Serialize;
use serde_yaml;
use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::Rng;

#[derive(Serialize)]
struct SimpleKeyVal{
    key: String,
    val: Vec<f64>
}

pub fn insert_vector(vec: &Vec<f64>){
    let vals = SimpleKeyVal{
        key: format!("input_{}", generate_unique_id().to_string()),
        val: vec.to_vec(),
    };

    let mut data:HashMap<String, Vec<f64>> = HashMap::new();

    data.insert(vals.key.clone(), vals.val.clone());

    let yaml = serde_yaml::to_string(&data).unwrap();

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.yaml");

    file.expect("Failed to open file")
        .write_all(yaml.as_bytes())
        .expect("Failed to write to YAML");
}

fn generate_unique_id() -> String {
    let mut rng = rand::thread_rng();

    // Generate a random number between 0 and 999,999 (inclusive)
    let random_number = rng.gen_range(0..=999_999);

    // Format the random number as a 6-digit code with leading zeros
    format!("{:06}", random_number)
}