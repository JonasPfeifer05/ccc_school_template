use ccc_school_39::Level::Level1;
use ccc_school_39::LevelRunner;

fn main() {
    LevelRunner::new(Level1).run(run_single_input, false);
}

fn run_single_input(context: String) -> String {
    println!("{}", context);
    context
}