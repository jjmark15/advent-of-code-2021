use std::num::ParseIntError;
use std::path::Path;

use crate::domain::day_0::Day0SolutionExecutor;
use crate::domain::day_1::Day1SolutionExecutor;
use crate::ports::cli::structopt::inputs::Lines;
use crate::ports::cli::structopt::outputs::List;
use crate::ports::cli::structopt::read_input;

pub(crate) fn run_day_0(part: u8, input_path: &Path) -> String {
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

pub(crate) fn run_day_1(part: u8, input_path: &Path) -> String {
    let executor = Day1SolutionExecutor::new();
    let input: Lines<String> = read_input(input_path).unwrap();
    let numbers: Vec<u64> = input
        .inner()
        .into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .unwrap();
    match part {
        1 => {
            let output = executor.part_1(numbers);
            output.to_string()
        }
        2 => {
            let output = executor.part_2(numbers);
            output.to_string()
        }
        _ => unimplemented!(),
    }
}
