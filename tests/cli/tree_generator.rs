use crate::helpers::generate_base_path;
use glob::glob;
use project_tree_manager::tree_generator::{Tree, TreeGenerator};
use std::fs;

#[test]
fn create_tree_from_yaml() {
    let base_path = generate_base_path("./dist");

    let body = r#"
---
- name: project
  kind: folder
  childs:
  - name: test
    src: git@github.com:lperdereau/project-tree-manager.git
    kind: project

- name: test
  src: git@github.com:lperdereau/project-tree-manager.git
  kind: project
"#;

    let tree = Tree::parse_from_yaml(body);
    TreeGenerator::create(&tree, &base_path);

    let paths = glob(&format!("{path}/*", path = &base_path)).expect("Failed to parse pattern");
    assert_eq!(paths.count(), tree.count());
    let _ = fs::remove_dir_all(&base_path);
}

#[test]
fn create_tree_from_json() {
    let base_path = generate_base_path("./dist");

    let body = r#"
[
    {
        "name": "project",
        "kind": "folder",
        "childs": [
            {
                "name": "test",
                "src": "git@github.com:lperdereau/project-tre-manager.git",
                "kind": "project"
            }
        ]
    },
    {
        "name": "test",
        "src": "git@github.com:lperdereau/project-tre-manager.git",
        "kind": "project"
    }
]"#;

    let tree = Tree::parse_from_json(body);
    TreeGenerator::create(&tree, &base_path);

    let paths = glob(&format!("{path}/*", path = &base_path)).expect("Failed to parse pattern");
    assert_eq!(paths.count(), tree.count());
    fs::remove_dir_all(&base_path).expect("Error to remove folder");
}
