use std::fmt::Display;
use std::fs;

mod level_executor;

const PATH_TO_RESOURCES: &str = "src/resources";

pub enum Level {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Level::Level1 => "level1",
            Level::Level2 => "level2",
            Level::Level3 => "level3",
            Level::Level4 => "level4",
            Level::Level5 => "level5",
        };

        f.write_str(value)
    }
}

pub struct LevelRunner {
    level: Level,
}

impl LevelRunner {
    pub fn new(level: Level) -> Self {
        Self {
            level
        }
    }

    pub fn run(&self, run_single_file_fn: fn(String) -> String, only_examples: bool) {
        fs::read_dir(format!("{}/{}", PATH_TO_RESOURCES, self.level))
            .expect("Failed to read level directory!")
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if only_examples {
                    entry.file_name().to_str().unwrap().contains("example")
                } else {
                    true
                }
            })
            .map(|entry| entry.path())
            .map(|entry| (entry.clone(), run_single_file_fn(fs::read_to_string(entry).unwrap())))
            .for_each(|(mut path, content)| {
                path.set_extension("out");
                println!("Writing to {:?}!", path.as_path());
                fs::write(path, content).unwrap();
            });
    }
}