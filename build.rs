use std::{
    env,
    error::Error,
    fs::{write, File},
    io::{Read, Write},
    path::Path,
};

use walkdir::WalkDir;
use zip::write::FileOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => {
            write(format!("{}/source.zip", env::var("OUT_DIR")?), "")?;
        }
        "release" => {
            let writer = File::create(format!("{}/source.zip", env::var("OUT_DIR")?))?;
            let mut zip = zip::ZipWriter::new(writer);
            let options = FileOptions::default().unix_permissions(0o755);

            let mut buffer = Vec::new();
            let prefix = "";
            for entry in WalkDir::new("src").into_iter().flatten() {
                let path = entry.path();
                let name = path.strip_prefix(Path::new(prefix)).unwrap();
                // Write file or directory explicitly
                // Some unzip tools unzip files with directory paths correctly, some do not!
                if entry.metadata().unwrap().is_file() {
                    println!("adding file {:?} as {:?} ...", path, name);
                    #[allow(deprecated)]
                    zip.start_file_from_path(name, options)?;
                    let mut f = File::open(path.canonicalize()?)?;

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&buffer)?;
                    buffer.clear();
                } else if !name.as_os_str().is_empty() {
                    // Only if not root! Avoids path spec / warning
                    // and mapname conversion failed error on unzip
                    println!("adding dir {:?} as {:?} ...", path, name);
                    #[allow(deprecated)]
                    zip.add_directory_from_path(name, options)?;
                }
            }
            let extra_source_files = vec!["Cargo.toml", "Cargo.lock", "README.md", "build.rs"];
            for file in extra_source_files {
                let path = Path::new(file);
                let name = path.strip_prefix(Path::new(prefix)).unwrap();
                println!("adding file {:?} as {:?} ...", path, name);
                #[allow(deprecated)]
                zip.start_file_from_path(name, options)?;
                let mut f = File::open(path)?;
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                buffer.clear();
            }
            zip.finish()?;
        }
        _ => (),
    }

    Result::Ok(())
}
