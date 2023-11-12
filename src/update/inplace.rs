use std::fs::File;
use std::io::{Read, Write};
use std::{env, fs};
use xz::read::XzDecoder;

pub fn uncompress_binary(buf: Vec<u8>) -> Vec<u8> {
    let mut decoder = XzDecoder::new(&buf[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();
    decompressed_data
}

pub fn update_binary_file(buf: Vec<u8>) {
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
