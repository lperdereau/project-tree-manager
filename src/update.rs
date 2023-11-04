use reqwest::header;
use self_update::cargo_crate_version;
use self_update::update::Release;
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

    display_changelog(&releases[0]);

    let data = download_asset(&releases[0], Option::None);
    let sum = get_sum(&releases[0], &get_current_target());
    if !check_sum(data.as_slice(), sum.unwrap()) {
        return;
    }

    let binary = uncompress_binary(data);
    update_binary_file(binary);

    println!("Up to date ✅");
}

fn get_current_target() -> String {
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}

fn download_asset(release: &Release, name: Option<&str>) -> Vec<u8> {
    let current_target = get_current_target();
    let target: &str = match name {
        Some(value) => value,
        None => current_target.as_str().as_ref(),
    };

    let asset = release.asset_for(target, Option::None).unwrap();

    let mut c = Cursor::new(Vec::new());

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
    out
}

fn uncompress_binary(buf: Vec<u8>) -> Vec<u8> {
    let mut decoder = XzDecoder::new(&buf[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    decompressed_data
}

fn update_binary_file(buf: Vec<u8>) {
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
        .write_all(buf.as_slice())
        .expect("Failed to write new version of binary.");

    output_file
        .set_permissions(
            File::open(&origin)
                .unwrap()
                .metadata()
                .unwrap()
                .permissions(),
        )
        .unwrap();
    fs::rename(output_path, origin).unwrap();
}

fn get_sum(release: &Release, target: &str) -> Option<String> {
    let checksum = download_asset(release, Option::Some("sha256sum"));
    for line in std::str::from_utf8(&checksum).unwrap().split('\n') {
        if line.contains(target) {
            let sum = line.chars().take_while(|&ch| ch != ' ').collect::<String>();
            return Option::Some(sum);
        }
    }
    Option::None
}

fn check_sum(buf: &[u8], sum: String) -> bool {
    let digest = sha256::digest(buf);
    if digest != sum {
        println!("❌ Sums are differents.");
        println!("");
        println!("Downloaded: {}", digest);
        println!("Source: {}", sum);
        return false;
    }
    true
}

fn display_changelog(release: &Release) {
    println!("");
    println!("");
    termimad::print_text(release.body.clone().unwrap().replace("\n\n", "\n").as_str());
}
