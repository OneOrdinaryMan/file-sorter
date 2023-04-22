use std::{fs, fs::DirEntry, io};

struct ClassifiedFiles {
    document_files: Vec<DirEntry>,
    picture_files: Vec<DirEntry>,
    music_files: Vec<DirEntry>,
    video_files: Vec<DirEntry>,
    compressed_files: Vec<DirEntry>,
}

impl ClassifiedFiles {
    fn get() -> io::Result<Vec<DirEntry>> {
        Ok(fs::read_dir(".")?
            .flatten()
            .filter(|x| x.path().is_file())
            .collect())
    }

    fn new() -> ClassifiedFiles {
        let files = match ClassifiedFiles::get() {
            Ok(val) => val,
            _ => Vec::new(),
        };

        let mut document_files: Vec<DirEntry> = Vec::new();
        let mut picture_files: Vec<DirEntry> = Vec::new();
        let mut music_files: Vec<DirEntry> = Vec::new();
        let mut video_files: Vec<DirEntry> = Vec::new();
        let mut compressed_files: Vec<DirEntry> = Vec::new();
        let document_ext = [
            Some(String::from("txt")),
            Some(String::from("text")),
            Some(String::from("log")),
            Some(String::from("docx")),
            Some(String::from("pdf")),
            Some(String::from("doc")),
            Some(String::from("asc")),
        ];
        let picture_ext = [
            Some(String::from("gif")),
            Some(String::from("png")),
            Some(String::from("bpg")),
            Some(String::from("jpeg")),
            Some(String::from("jpg")),
            Some(String::from("tif")),
            Some(String::from("icon")),
        ];
        let music_ext = [
            Some(String::from("wav")),
            Some(String::from("mp3")),
            Some(String::from("flac")),
            Some(String::from("ogg")),
            Some(String::from("m4a")),
            Some(String::from("midi")),
            Some(String::from("mid")),
        ];
        let video_ext = [
            Some(String::from("mp4")),
            Some(String::from("mkv")),
            Some(String::from("webm")),
            Some(String::from("3gp")),
            Some(String::from("mov")),
            Some(String::from("ts")),
        ];
        let compressed_ext = [
            Some(String::from("zip")),
            Some(String::from("rar")),
            Some(String::from("gzip")),
            Some(String::from("tar")),
            Some(String::from("tar.xz")),
            Some(String::from("tar.gz")),
        ];

        for file in files {
            if document_ext.contains(&match file.path().extension() {
                Some(val) => val.to_os_string().into_string().ok(),
                None => None,
            }) {
                document_files.push(file);
            } else if music_ext.contains(&match file.path().extension() {
                Some(val) => val.to_os_string().into_string().ok(),
                None => None,
            }) {
                music_files.push(file);
            } else if video_ext.contains(&match file.path().extension() {
                Some(val) => val.to_os_string().into_string().ok(),
                None => None,
            }) {
                video_files.push(file);
            } else if picture_ext.contains(&match file.path().extension() {
                Some(val) => val.to_os_string().into_string().ok(),
                None => None,
            }) {
                picture_files.push(file);
            } else if compressed_ext.contains(&match file.path().extension() {
                Some(val) => val.to_os_string().into_string().ok(),
                None => None,
            }) {
                compressed_files.push(file);
            }
        }
        ClassifiedFiles {
            document_files,
            picture_files,
            music_files,
            video_files,
            compressed_files,
        }
    }
    fn pretty_print(&self) {
        println!("{:#?}", self.document_files);
        println!("{:#?}", self.picture_files);
        println!("{:#?}", self.music_files);
        println!("{:#?}", self.video_files);
        println!("{:#?}", self.compressed_files);
    }
}

fn main() {
    let files = ClassifiedFiles::new();
    files.pretty_print();
}
