//! # File Sorter
//! Sorts the files into different directories
//! Simply run the binary in the directory to be sorted.
//! **Will not sort sub directories.**
pub mod file_fetcher {
    //! fetches the files and classifies them
    use std::{fs, fs::DirEntry, io};
    pub struct ClassifiedFiles {
        pub document_files: Vec<DirEntry>,
        pub picture_files: Vec<DirEntry>,
        pub music_files: Vec<DirEntry>,
        pub video_files: Vec<DirEntry>,
        pub compressed_files: Vec<DirEntry>,
    }

    impl ClassifiedFiles {
        fn get() -> io::Result<Vec<DirEntry>> {
            Ok(fs::read_dir(".")?
                .flatten()
                .filter(|x| x.path().is_file())
                .collect())
        }

        pub fn new() -> ClassifiedFiles {
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
                Some(String::from("epub")),
                Some(String::from("cbz")),
                Some(String::from("cbr")),
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
        pub fn pretty_print(&self) {
            println!("{:#?}", self.document_files);
            println!("{:#?}", self.picture_files);
            println!("{:#?}", self.music_files);
            println!("{:#?}", self.video_files);
            println!("{:#?}", self.compressed_files);
        }
    }
}

pub mod file_operations {
    //! Creating the directories and moving the files in.
    use super::file_fetcher;
    use std::{fs, path::Path, process};
    pub fn create_directories(&directories: &[&Path; 5]) {
        if Path::new(directories[0]).exists()
            || Path::new(directories[1]).exists()
            || Path::new(directories[2]).exists()
            || Path::new(directories[3]).exists()
            || Path::new(directories[4]).exists()
        {
            println!("Fatal! one of  {:?} exists!", directories);
            process::exit(1);
        }

        for dir in directories {
            fs::create_dir(Path::new(dir)).expect("Failed to create directory");
        }
    }
    pub fn move_files(&directories: &[&Path; 5]) {
        let files = file_fetcher::ClassifiedFiles::new();
        //files.pretty_print();
        if !files.document_files.is_empty() {
            for file in files.document_files {
                fs::rename(
                    file.path(),
                    directories[0].join(match file.path().strip_prefix("./") {
                        Ok(value) => value.to_path_buf(),
                        Err(e) => {
                            println!("{:?}", e);
                            file.path()
                        }
                    }),
                )
                .expect("Cannot move the file");
            }
        } else {
            println!("There are no documents");
            fs::remove_dir(directories[0]).expect("Cannot remove directory");
        }
        if !files.picture_files.is_empty() {
            for file in files.picture_files {
                fs::rename(
                    file.path(),
                    directories[1].join(match file.path().strip_prefix("./") {
                        Ok(value) => value.to_path_buf(),
                        Err(e) => {
                            println!("{:?}", e);
                            file.path()
                        }
                    }),
                )
                .expect("Cannot move the file");
            }
        } else {
            println!("There are no pictures");
            fs::remove_dir(directories[1]).expect("Cannot remove directory");
        }
        if !files.music_files.is_empty() {
            for file in files.music_files {
                fs::rename(
                    file.path(),
                    directories[2].join(match file.path().strip_prefix("./") {
                        Ok(value) => value.to_path_buf(),
                        Err(e) => {
                            println!("{:?}", e);
                            file.path()
                        }
                    }),
                )
                .expect("Cannot move the file");
            }
        } else {
            println!("There are no music files");
            fs::remove_dir(directories[2]).expect("Cannot remove directory");
        }
        if !files.video_files.is_empty() {
            for file in files.video_files {
                fs::rename(
                    file.path(),
                    directories[3].join(match file.path().strip_prefix("./") {
                        Ok(value) => value.to_path_buf(),
                        Err(e) => {
                            println!("{:?}", e);
                            file.path()
                        }
                    }),
                )
                .expect("Cannot move the file");
            }
        } else {
            println!("There are no videos");
            fs::remove_dir(directories[3]).expect("Cannot remove directory");
        }
        if !files.compressed_files.is_empty() {
            for file in files.compressed_files {
                fs::rename(
                    file.path(),
                    directories[4].join(match file.path().strip_prefix("./") {
                        Ok(value) => value.to_path_buf(),
                        Err(e) => {
                            println!("{:?}", e);
                            file.path()
                        }
                    }),
                )
                .expect("Cannot move the file");
            }
        } else {
            println!("There are no compressed files");
            fs::remove_dir(directories[4]).expect("Cannot remove directory");
        }
    }
}
