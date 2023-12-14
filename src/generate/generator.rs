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

pub fn record(file_type: &str, folder_path: &str) -> String {
    let tree = TreeGenerator::read(folder_path);
    match file_type {
        "json" => tree.to_json(),
        "yaml" => tree.to_yaml(),
        _ => panic!("FileType unknown"),
    }
}

pub struct TreeGenerator {}

impl TreeGenerator {
    pub fn create(tree: &Tree, base_path: &str) {
        for tree_elements in &tree.childs {
            let _ = tree_elements.create(base_path);
        }
    }

    pub fn read(base_path: &str) -> Tree {
        let mut tree = Tree::new();
        tree.build_from_path(base_path);
        tree
    }
}
