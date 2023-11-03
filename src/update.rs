use reqwest::header;
use self_update::cargo_crate_version;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{env, fs};
use xz::read::XzDecoder;

pub fn update(force: bool) {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("lperdereau")
        .repo_name("project-tree-manager")
        .build()
        .unwrap()
        .fetch()
        .unwrap();

    if !force && releases[0].version == cargo_crate_version!() {
        println!("Already up to date ✅");
        return;
    }

    let asset = releases[0].asset_for(&get_target(), Option::None).unwrap();

    let mut c = Cursor::new(Vec::new());

    println!("Downloading: {}:{}", asset.name, releases[0].version);
    termimad::print_text(
        releases[0]
            .body
            .clone()
            .unwrap()
            .replace("\n\n", "\n")
            .as_str(),
    );

    self_update::Download::from_url(&asset.download_url)
        .set_header(
            header::ACCEPT,
            "application/octet-stream"
                .parse()
                .expect("Failed to prepare download request."),
        )
        .download_to(std::io::Read::by_ref(&mut c))
        .unwrap();

    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    let mut decoder = XzDecoder::new(&out[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    let mut output_path = env::current_exe().unwrap();
    let origin = output_path.clone();

    output_path.set_file_name(format!(
        ".{}.tmp",
        env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_os_string()
            .to_str()
            .unwrap()
    ));

    let mut output_file = File::create(&output_path).unwrap();
    output_file
        .write_all(decompressed_data.as_slice())
        .expect("Failed to write new version of binary.");

    output_file.set_permissions(
        File::open(&origin)
            .unwrap()
            .metadata()
            .unwrap()
            .permissions(),
    ).unwrap();
    fs::rename(output_path, origin).unwrap();

    println!("Up to date ✅");
}

fn get_target() -> String {
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}
