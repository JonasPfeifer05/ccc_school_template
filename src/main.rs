use crate::level_reader::run_level;

mod level_1;
mod level_2;
mod level_3;
mod level_4;
mod level_5;
mod level_reader;

fn main() {
    let level = 3;
    let run_example = false;

    match level {
        1 => run_level("level1", run_example, level_1::level_1),
        2 => run_level("level2", run_example, level_2::level_2),
        3 => run_level("level3", run_example, level_3::level_3),
        4 => run_level("level4", run_example, level_4::level_4),
        5 => run_level("level5", run_example, level_5::level_5),
        _ => unreachable!()
    }
}
