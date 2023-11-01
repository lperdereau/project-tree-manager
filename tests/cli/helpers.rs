use std::fs;
use uuid::Uuid;

pub fn generate_base_path(base_path: &str) -> String {
    if fs::try_exists(&base_path).is_err() {
        fs::create_dir(&base_path).expect("error during creation random directory");
    }

    let uuid = Uuid::new_v4();
    let folder_path = format!("{base_path}/{uuid}");
    fs::create_dir(&folder_path).expect("error during creation random directory");
    folder_path
}
