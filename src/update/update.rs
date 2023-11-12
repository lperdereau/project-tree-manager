use crate::utils::{error_chain_fmt, get_current_target};
use super::download::download_asset;
use super::inplace::{update_binary_file, uncompress_binary};
use self_update::{cargo_crate_version, update::Release};
use std::process;

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
        process::exit(0);
    }

    display_changelog(&releases[0]);

    let data = download_asset(&releases[0], Option::None);
    let sum = get_sum(&releases[0], &get_current_target()).unwrap();
    let digest = sha256::digest(data.as_slice());

    if digest != sum {
        println!("{}", UpdateError::InvalidSum(digest, sum));
        process::exit(1);
    }

    let binary = uncompress_binary(data);
    update_binary_file(binary);

    println!("Up to date ✅");
    process::exit(0);
}

fn display_changelog(release: &Release) {
    println!("");
    println!("");
    termimad::print_text(release.body.clone().unwrap().replace("\n\n", "\n").as_str());
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

#[derive(thiserror::Error)]
enum UpdateError {
    #[error(
        r#"❌ Sums are differents.
Downloaded: {0}
Source: {1}"#
    )]
    InvalidSum(String, String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
