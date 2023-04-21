use std::{fs, fs::DirEntry};

fn main() {
    let files: Vec<DirEntry> = fs::read_dir(".")
        .unwrap()
        .flatten()
        .filter(|x| x.path().is_file())
        .collect();
    let mut document_files: Vec<DirEntry> = Vec::new();
    let mut media_files: Vec<DirEntry> = Vec::new();
    let mut pictures_files: Vec<DirEntry> = Vec::new();
    let document_ext = [
        Some(String::from("pdf")),
        Some(String::from("epub")),
        Some(String::from("docx")),
    ];
    let media_ext = [
        Some(String::from("mp4")),
        Some(String::from("mkv")),
        Some(String::from("mp3")),
        Some(String::from("flac")),
    ];
    let pictures_ext = [
        Some(String::from("jpg")),
        Some(String::from("png")),
        Some(String::from("gif")),
    ];
    println!("{:?}", files);
    for file in files {
        if document_ext.contains(&match file.path().extension() {
            Some(val) => val.to_os_string().into_string().ok(),
            None => None,
        }) {
            document_files.push(file);
        } else if media_ext.contains(&match file.path().extension() {
            Some(val) => val.to_os_string().into_string().ok(),
            None => None,
        }) {
            media_files.push(file);
        } else if pictures_ext.contains(&match file.path().extension() {
            Some(val) => val.to_os_string().into_string().ok(),
            None => None,
        }) {
            pictures_files.push(file);
        }
    }
    println!("{:?}", document_files);
    println!("{:?}", media_files);
    println!("{:?}", pictures_files);
}
