use std::fs::read_to_string;
use std::path::Path;

use super::paths::get_exe_folder;

pub fn read_string(folder: &str, path: &str) -> String {
    let file_path = Path::new(&get_exe_folder()).join(folder).join(path);
    read_to_string(file_path).unwrap()
}
