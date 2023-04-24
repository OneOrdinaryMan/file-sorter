use one_ordinary_man_file_sorter::file_operations;
use std::path::Path;

fn main() {
    // Declaring the directory variable here.
    let directories: [&Path; 5] = [
        Path::new("Documents"),
        Path::new("Pictures"),
        Path::new("Music"),
        Path::new("Videos"),
        Path::new("Compressed"),
    ];
    // Calling create_directory function and then calling the move file.
    file_operations::create_directories(&directories);
    file_operations::move_files(&directories);
}
