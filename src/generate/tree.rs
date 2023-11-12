use serde::Deserialize;
use std::{fs, path::Path, str::FromStr, vec};

pub struct Tree {
    pub(crate) childs: Vec<TreeElement>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TreeElement {
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

    pub fn childs(&self) -> &Vec<TreeElement> {
        &self.childs
    }
}

impl TreeElement {
    pub fn create(&self, base_path: &str) {
        match self {
            TreeElement::Folder(folder) => TreeElement::create_folder(folder, base_path),
            TreeElement::Project(project) => TreeElement::create_project(project, base_path),
        };
    }

    fn create_folder(folder: &Folder, base_path: &str) {
        let path = std::path::Path::new(base_path)
            .join(&folder.name)
            .as_path()
            .display()
            .to_string();

        fs::create_dir(&path).expect("Error to create folder");

        for tree_elements in &folder.childs {
            tree_elements.create(&path);
        }
    }

    fn create_project(project: &Project, base_path: &str) {
        let path = std::path::Path::new(base_path)
            .join(&project.name)
            .as_path()
            .display()
            .to_string();

        let mut cb = git2::RemoteCallbacks::new();
        let git_config = git2::Config::open_default().unwrap();
        let mut ch = git2_credentials::CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(cb)
            .download_tags(git2::AutotagOption::All)
            .update_fetchhead(true);

        git2::build::RepoBuilder::new()
            .fetch_options(fo)
            .clone(project.source.as_str(), std::path::Path::new(&path))
            .expect("Error to clone repository");
    }
}
