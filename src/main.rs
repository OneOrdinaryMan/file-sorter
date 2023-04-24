use file_sorter::file_operations;
use std::path::Path;

fn main() {
    let directories: [&Path; 5] = [
        Path::new("Documents"),
        Path::new("Pictures"),
        Path::new("Music"),
        Path::new("Videos"),
        Path::new("Compressed"),
    ];
    file_operations::create_directories(&directories);
    file_operations::move_files(&directories);
}
