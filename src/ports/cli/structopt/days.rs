use std::path::Path;

use crate::domain::day_0::Day0SolutionExecutor;
use crate::domain::day_1::Day1SolutionExecutor;
use crate::domain::solution_executor::SolutionExecutor;
use crate::ports::cli::structopt::day_part::DayPart;
use crate::ports::cli::structopt::inputs::Lines;
use crate::ports::cli::structopt::outputs::List;
use crate::ports::cli::structopt::{read_input, read_input_str};

pub(crate) fn run_day_0(part: DayPart, input_path: &Path) -> String {
    let executor = Day0SolutionExecutor::new();
    match part {
        DayPart::One => {
            let input: Lines<String> = read_input(input_path).unwrap();
            let output = List::new(executor.part_1(input.inner()));
            output.to_string()
        }
        DayPart::Two => unimplemented!(),
    }
}

pub(crate) fn run_day_1(part: DayPart, input_path: &Path) -> String {
    let executor = Day1SolutionExecutor::new();
    let input: Lines<u64> = read_input_str(input_path).unwrap();
    match part {
        DayPart::One => {
            let output = executor.part_1(input.inner());
            output.to_string()
        }
        DayPart::Two => {
            let output = executor.part_2(input.inner());
            output.to_string()
        }
    }
}
