use serde::Deserialize;
use std::fs;

pub struct Tree {
    childs: Vec<TreeElement>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum TreeElement {
    Project(Project),
    Folder(Folder),
}

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(alias = "src")]
    source: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    name: String,
    childs: Vec<TreeElement>,
}

impl Folder {
    pub fn count(&self) -> usize {
        let mut count = 0;
        for tree_elements in &self.childs {
            match tree_elements {
                TreeElement::Folder(folder) => count += folder.count(),
                TreeElement::Project(_) => count += 1,
            }
        }
        count
    }
}

impl Tree {
    pub fn parse_from_json(content: &str) -> Self {
        let childs: Vec<TreeElement> = serde_json::from_str(content).unwrap();
        Self { childs }
    }

    pub fn parse_from_yaml(content: &str) -> Self {
        let childs: Vec<TreeElement> = serde_yaml::from_str(content).unwrap();
        Self { childs }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for tree_elements in &self.childs {
            match tree_elements {
                TreeElement::Folder(folder) => count += folder.count(),
                TreeElement::Project(_) => count += 1,
            }
        }
        count
    }
}

impl TreeElement {
    fn create(&self, base_path: &str) -> Result<(), std::io::Error> {
        let result = match self {
            TreeElement::Folder(folder) => TreeElement::create_folder(folder, base_path),
            TreeElement::Project(project) => TreeElement::create_project(project, base_path),
        };
        result
    }

    fn create_folder(folder: &Folder, base_path: &str) -> Result<(), std::io::Error> {
        let result = fs::create_dir(format!(
            "{base_path}/{folder_name}",
            folder_name = folder.name
        ));

        let folder_path = format!("{base_path}/{folder_name}", folder_name = folder.name);
        for tree_elements in &folder.childs {
            return tree_elements.create(&folder_path);
        }
        result
    }

    fn create_project(project: &Project, base_path: &str) -> Result<(), std::io::Error> {
        fs::create_dir(format!(
            "{base_path}/{project_name}",
            project_name = project.name
        ))
    }
}

pub struct TreeGenerator {}

impl TreeGenerator {
    pub fn create(tree: &Tree, base_path: &str) {
        for tree_elements in &tree.childs {
            let _ = tree_elements.create(base_path);
        }
    }
}
