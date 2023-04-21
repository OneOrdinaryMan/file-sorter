use std::{fs, fs::DirEntry, io};

fn files_in_dir() -> io::Result<Vec<DirEntry>> {
    fs::read_dir(".")?
        .flatten()
        .filter(|x| !x.path().is_dir())
        .map(|x| Ok(x))
        .collect()
}

fn toml_filter(files: Vec<DirEntry>) -> Vec<DirEntry> {
    files
        .into_iter()
        .filter(|x| {
            Some(String::from("toml"))
                == match x.path().extension() {
                    Some(val) => val.to_os_string().into_string().ok(),
                    None => None,
                }
        })
        .collect()
}
fn main() {
    let files = files_in_dir();
    //println!("{:?}", files);
    let ok_files = match files {
        Ok(val) => val,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    };
    //println!("{:?}", ok_files)
    let toml_file = toml_filter(ok_files);
    println!("{:?}", toml_file);
}
