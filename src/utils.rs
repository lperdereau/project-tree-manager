use serde_json;
use serde_yaml;
use std::fs::File;
use std::io::Read;

pub enum FileType {
    JSON(String),
    YAML(String),
    Unknown,
}

pub fn get_file_type(file_path: &str) -> FileType {
    let mut file = File::open(file_path).expect("Unable to open the file.");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error during reading the file.");

    if serde_json::from_str::<serde_json::Value>(&content).is_ok() {
        return FileType::JSON(content);
    } else if serde_yaml::from_str::<serde_yaml::Value>(&content).is_ok() {
        return FileType::YAML(content);
    } else {
        return FileType::Unknown;
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn get_current_target() -> String {
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}
