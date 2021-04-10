use std::env;
use std::fs;
use std::path::Path;

// "Share" code with the actual project
#[path = "src/society/paths.rs"]
mod easy_path;
use easy_path::EasyPath;

#[allow(dead_code)]
fn print<S: Into<String>>(message: S) {
    println!("{}", format!("cargo:warning={}", message.into()));
}

fn copy_all_with_extension(src: &Path, dest: &str, extension: &str, force_update: bool) -> Result<(), std::io::Error> {
    let mut created_folder = false;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_all_with_extension(&path, Path::new(&dest).join(path.file_name().unwrap()).stringify(), extension, force_update)?;
        } else if let Some(file_name) = path.file_name() {
            if let Some(file_extension) = path.extension() {
                if file_extension.stringify().to_ascii_lowercase() == extension || extension == "*" {
                    let dest_file = Path::new(&dest).join(file_name);

                    if !dest_file.exists() || force_update {
                        if !created_folder {
                            fs::create_dir_all(dest).expect("Unable to create output dir");
                            created_folder = true;
                        }
                        // Joys, no way to do this easily: https://github.com/rust-lang/cargo/issues/5305
                        fs::copy(path, dest_file)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("No OUT_DIR set?");
    // ../../.. to get to out of the crate specific folder
    let dest_dir = Path::new(&out_dir).join("..").join("..").join("..");

    let platform = env::var("CARGO_CFG_TARGET_OS").expect("No Target OS?");
    if let "windows" = platform.as_str() {
        let lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("lib").join("win");

        let mut res = winres::WindowsResource::new();
        // Now causing window to be 4x smaller on laptop, so not needed now? Not sure what changed.
        //res.set_manifest_file(lib_dir.join("incremental_society.manifest").stringify());
        res.set_icon(lib_dir.join("incremental_society.ico").stringify());

        res.compile().expect("Unable to run windows resource compiler");
    }

    copy_all_with_extension(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("data"),
        &dest_dir.join("data").stringify(),
        "json",
        true,
    )
    .unwrap_or_else(|_| panic!("Unable to copy data"));
}
