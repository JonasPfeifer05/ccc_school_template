use std::fs;
use std::path::{Path, PathBuf};

pub fn run_level(level: &str, run_only_example: bool, main: fn(String) -> String) {
    fs::read_dir(format!("src/resources/{}", level))
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if run_only_example {
                entry.file_name().to_str().unwrap().contains("example")
            } else {
                true
            }
        })
        .map(|entry| entry.path())
        .map(|entry| (entry.clone(), main(fs::read_to_string(entry).unwrap())))
        .for_each(|(mut path, content)| {
            path.set_extension("out");
            println!("Writing to {:?}!", path.as_path());
            fs::write(path, content).unwrap();
        });
}