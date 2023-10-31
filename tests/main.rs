use glob::glob;
use project_tree_manager::tree_generator::TreeGenerator;

#[test]
fn create_tree_from_yaml() {
    let body = r#"
---
- name: ./dist/
  kind: folder
  childs:
  - name: test
    src: git@github.com:lperdereau/project-tree-manager.git
    kind: project
"#;

    let tg = TreeGenerator::create_from_yaml(body);

    let paths = glob("./dist/**/*").expect("Failed to parse pattern");
    assert_eq!(paths.count(), tg.count());
}

#[test]
fn create_tree_from_json() {
    let body = r#"
[
    {
        "name": "./dist/",
        "kind": "folder",
        "childs": [
        {
            "name": "test",
            "src": "git@github.com:lperdereau/project-tre-manager.git",
            "kind": "project"
        }
        ]
    }
]"#;

    TreeGenerator::create_from_json(body);

    let paths = glob("./dist/*").expect("Failed to parse pattern");
    assert_eq!(paths.count(), 1);
}
