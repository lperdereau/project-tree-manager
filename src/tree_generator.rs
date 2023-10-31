use serde::Deserialize;

pub struct TreeGenerator {
    trees: Vec<TreeElement>,
}

#[derive(Debug, Deserialize)]
#[serde(tag="kind", rename_all="lowercase")]
enum TreeElement {
    Project(Project),
    Folder(Folder),
}

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(alias="src")]
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
        };
        count
    }
}

impl TreeGenerator {
    pub fn create_from_json(content: &str) -> Self {
        let trees: Vec<TreeElement> = serde_json::from_str(content).unwrap();
        Self { trees }
    }

    pub fn create_from_yaml(content: &str) -> Self {
        let trees: Vec<TreeElement> = serde_yaml::from_str(content).unwrap();
        Self { trees }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for tree_elements in &self.trees {
            match tree_elements {
                TreeElement::Folder(folder) => count += folder.count(),
                TreeElement::Project(_) => count += 1,
            }
        };
        count
    }
}
