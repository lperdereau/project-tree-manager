use crate::file::{get_file_type, FileType};
use crate::tree_generator;

pub fn generate(file_path: &str, folder_path: &str) {
    let tree = match get_file_type(file_path) {
        FileType::JSON(body) => tree_generator::Tree::parse_from_json(&body),
        FileType::YAML(body) => tree_generator::Tree::parse_from_yaml(&body),
        _ => panic!("File type not recognized, json and yaml are accepted."),
    };

    tree_generator::TreeGenerator::create(&tree, folder_path)
}
