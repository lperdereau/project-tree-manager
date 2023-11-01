use std::fs::File;
use std::io::Read;
use serde_json;
use serde_yaml;

pub enum FileType {
    JSON(String),
    YAML(String),
    Unknown,
}

pub fn get_file_type(file_path: &str) -> FileType {
    let mut file = File::open(file_path).expect("Unable to open the file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Error during reading the file.");

    if serde_json::from_str::<serde_json::Value>(&content).is_ok() {
        return FileType::JSON(content);
    } else if serde_yaml::from_str::<serde_yaml::Value>(&content).is_ok() {
        return FileType::YAML(content);
    } else {
        return FileType::Unknown;
    }
}
