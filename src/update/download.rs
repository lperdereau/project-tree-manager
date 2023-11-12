use crate::utils::get_current_target;
use reqwest::header;
use self_update::update::Release;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn download_asset(release: &Release, name: Option<&str>) -> Vec<u8> {
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
