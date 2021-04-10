use std::path::MAIN_SEPARATOR;

#[allow(dead_code)]
pub fn get_exe_folder() -> String {
    let exe = std::env::current_exe().unwrap();
    let exe_path = exe.to_str().unwrap();
    let mut bits: Vec<&str> = exe_path.split(MAIN_SEPARATOR).collect();
    bits.pop();

    #[cfg(test)]
    bits.pop();

    bits.join(&MAIN_SEPARATOR.to_string())
}

pub trait EasyPath {
    fn stringify(&self) -> &str;
    fn stringify_owned(&self) -> String;
}

impl EasyPath for std::path::Path {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}

impl EasyPath for std::path::PathBuf {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.stringify().to_string()
    }
}

impl EasyPath for &std::ffi::OsStr {
    fn stringify(&self) -> &str {
        self.to_str().unwrap()
    }
    fn stringify_owned(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
