use std::path::Path;

use crate::domain::day_0::Day0SolutionExecutor;
use crate::ports::cli::structopt::inputs::Lines;
use crate::ports::cli::structopt::outputs::List;
use crate::ports::cli::structopt::read_input;

pub(crate) fn run_day_1(part: u8, input_path: &Path) -> String {
    let executor = Day0SolutionExecutor::new();
    match part {
        1 => {
            let input: Lines<String> = read_input(input_path).unwrap();
            let output = List::new(executor.part_1(input.inner()));
            output.to_string()
        }
        _ => unimplemented!(),
    }
}
