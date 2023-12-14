use git2::{Remote, Repository};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{fs, vec, borrow::BorrowMut, sync::Arc};

#[derive(Clone)]
pub struct Tree {
    pub(crate) childs: Vec<TreeElement>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TreeElement {
    Project(Project),
    Folder(Folder),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    #[serde(alias = "src")]
    source: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Folder {
    name: String,
    childs: Vec<TreeElement>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name: name,
            childs: vec![],
        }
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

    fn get_folder(childs: Vec<TreeElement>, name: &str) -> Option<Folder> {
        for element in childs {
            if let TreeElement::Folder(folder) = element {
                if folder.name == name {
                    return Some(folder.clone());
                }
            }
        }
        None
    }

    pub fn get_or_create_folder(&mut self, name: &str) -> Folder {
        let childs = self.clone().childs;
        if let Some(folder) = Self::get_folder(childs, name) {
            return folder;
        }

        Folder::new(name.to_string())
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree { childs: vec![] }
    }
    pub fn parse_from_json(content: &str) -> Self {
        let childs: Vec<TreeElement> = serde_json::from_str(content).unwrap();
        Self { childs }
    }

    pub fn parse_from_yaml(content: &str) -> Self {
        let childs: Vec<TreeElement> = serde_yaml::from_str(content).unwrap();
        Self { childs }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.childs).unwrap()
    }

    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self.childs).unwrap()
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

    pub fn build_from_path(&mut self, path: &str) {
        let path = std::path::Path::new(path)
            .join("**/.git/")
            .as_path()
            .display()
            .to_string();

        for entry in glob(&path).expect("Failed to read glob pattern") {
            let path = entry.unwrap().display().to_string().replace(".git", "");
            let project_name = path.rsplitn(2, '/').collect::<Vec<&str>>()[0];
            let folder_path = path.rsplitn(2, '/').collect::<Vec<&str>>()[1];
            let repo = &Repository::open(&path).unwrap();
            let origin = Tree::get_remote(&repo).url().unwrap().to_string();
            let project = Project {
                name: project_name.to_string(),
                source: origin,
            };
            let mut folder = self.get_or_create_end_folder(&folder_path);
            folder.childs.push(TreeElement::Project(project))
        }
    }

    fn get_remote(repo: &Repository) -> Remote<'_> {
        let origin = repo.find_remote("origin");
        if origin.is_err() {
            let remotes = repo.remotes().unwrap();
            let remote = remotes.iter().next().unwrap().unwrap();
            return repo.find_remote(remote).unwrap();
        }
        origin.unwrap()
    }

    pub fn get_or_create_end_folder(&mut self, path: &str) -> Folder {
        let mut paths = path.split("/");
        let name = paths.next().unwrap();
        let mut folder = self.get_or_create_folder(name);
        for path in paths {
            let sub = folder.get_or_create_folder(path);
            folder.childs.push(TreeElement::Folder(sub.clone()));
            folder = sub;
        }
        self.childs.push(TreeElement::Folder(folder.clone()));
        folder
    }

    fn get_folder(childs: Vec<TreeElement>, name: &str) -> Option<Folder> {
        for element in childs {
            if let TreeElement::Folder(folder) = element {
                if folder.name == name {
                    return Some(folder.clone());
                }
            }
        }
        None
    }

    pub fn get_or_create_folder(&mut self, name: &str) -> Folder {
        let childs = self.clone().childs;
        if let Some(folder) = Self::get_folder(childs, name) {
            return folder;
        }

        Folder::new(name.to_string())
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
