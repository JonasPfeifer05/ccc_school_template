// use std::fs;
// use crate::Level;
//
// const PATH_TO_RESOURCES: &str = "src/resources";
// pub fn run_level(level: &Level, run_only_example: bool, callback: fn(String) -> String) {
//     fs::read_dir(format!("{}/{}", PATH_TO_RESOURCES, level))
//         .unwrap()
//         .filter_map(|entry| entry.ok())
//         .filter(|entry| {
//             if run_only_example {
//                 entry.file_name().to_str().unwrap().contains("example")
//             } else {
//                 true
//             }
//         })
//         .map(|entry| entry.path())
//         .map(|entry| (entry.clone(), main(fs::read_to_string(entry).unwrap())))
//         .for_each(|(mut path, content)| {
//             path.set_extension("out");
//             println!("Writing to {:?}!", path.as_path());
//             fs::write(path, content).unwrap();
//         });
// }