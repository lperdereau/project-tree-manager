use crate::utils::{get_file_type, FileType};
use super::Tree;

pub fn generate(file_path: &str, folder_path: &str) {
    let tree = match get_file_type(file_path) {
        FileType::JSON(body) => Tree::parse_from_json(&body),
        FileType::YAML(body) => Tree::parse_from_yaml(&body),
        _ => panic!("File type not recognized, json and yaml are accepted."),
    };

    TreeGenerator::create(&tree, folder_path)
}

pub struct TreeGenerator {}

impl TreeGenerator {
    pub fn create(tree: &Tree, base_path: &str) {
        for tree_elements in tree.childs() {
            let _ = tree_elements.create(base_path);
        }
    }
}
