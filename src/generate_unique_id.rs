use std::time::SystemTime;

pub fn generate_unique_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Error")
        .as_secs();
    format!("data/output_{}", timestamp)
}
