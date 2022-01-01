use std::path::Path;

use crate::domain::solution_executor::day_0::Day0SolutionExecutor;
use crate::domain::solution_executor::day_1::Day1SolutionExecutor;
use crate::domain::solution_executor::day_2::Day2SolutionExecutor;
use crate::domain::solution_executor::day_3::Day3SolutionExecutor;
use crate::domain::solution_executor::day_4::Day4SolutionExecutor;
use crate::domain::solution_executor::day_5::Day5SolutionExecutor;
use crate::domain::solution_executor::day_6::Day6SolutionExecutor;
use crate::domain::solution_executor::day_7::Day7SolutionExecutor;
use crate::domain::solution_executor::day_8::Day8SolutionExecutor;
use crate::domain::solution_executor::SolutionExecutor;
use crate::ports::cli::structopt::day_part::DayPart;
use crate::ports::cli::structopt::inputs::bingo_game::BingoGame;
use crate::ports::cli::structopt::inputs::comma_separated_list::CommaSeparatedList;
use crate::ports::cli::structopt::inputs::straight_line::StraightLine;
use crate::ports::cli::structopt::inputs::submarine_display_signals::SubmarineDisplaySignal;
use crate::ports::cli::structopt::inputs::{DirectionAndSize, Lines};
use crate::ports::cli::structopt::outputs::List;
use crate::ports::cli::structopt::{read_input, read_input_str};

pub(crate) fn run_day_0(part: DayPart, input_path: &Path) -> String {
    let executor = Day0SolutionExecutor::new();
    let input: Lines<String> = read_input(input_path).unwrap();
    match part {
        DayPart::One => List::new(executor.part_1(input.inner())).to_string(),
        DayPart::Two => unimplemented!(),
    }
}

pub(crate) fn run_day_1(part: DayPart, input_path: &Path) -> String {
    let executor = Day1SolutionExecutor::new();
    let input: Lines<u64> = read_input_str(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_2(part: DayPart, input_path: &Path) -> String {
    let executor = Day2SolutionExecutor::new();
    let input: Lines<DirectionAndSize> = read_input(input_path).unwrap();
    let domain_input: Vec<crate::domain::solution_executor::day_2::DirectionAndSize> = input
        .inner()
        .into_iter()
        .map(DirectionAndSize::into)
        .collect();
    match part {
        DayPart::One => executor.part_1(domain_input).to_string(),
        DayPart::Two => executor.part_2(domain_input).to_string(),
    }
}

pub(crate) fn run_day_3(part: DayPart, input_path: &Path) -> String {
    let executor = Day3SolutionExecutor::new();
    let input: Lines<String> = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_4(part: DayPart, input_path: &Path) -> String {
    let executor = Day4SolutionExecutor::new();
    let input: BingoGame = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.into()).to_string(),
        DayPart::Two => executor.part_2(input.into()).to_string(),
    }
}

pub(crate) fn run_day_5(part: DayPart, input_path: &Path) -> String {
    let executor = Day5SolutionExecutor::new();
    let input: Lines<StraightLine> = read_input(input_path).unwrap();
    let domain_input: Vec<crate::domain::solution_executor::day_5::StraightLine> =
        input.inner().into_iter().map(Into::into).collect();
    match part {
        DayPart::One => executor.part_1(domain_input).to_string(),
        DayPart::Two => executor.part_2(domain_input).to_string(),
    }
}

pub(crate) fn run_day_6(part: DayPart, input_path: &Path) -> String {
    let executor = Day6SolutionExecutor::new();
    let lines: Lines<CommaSeparatedList<u8>> = read_input_str(input_path).unwrap();
    let input = lines.inner().pop().unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_7(part: DayPart, input_path: &Path) -> String {
    let executor = Day7SolutionExecutor::new();
    let lines: Lines<CommaSeparatedList<u64>> = read_input_str(input_path).unwrap();
    let input = lines.inner().pop().unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_8(part: DayPart, input_path: &Path) -> String {
    let executor = Day8SolutionExecutor::new();
    let lines: Lines<SubmarineDisplaySignal> = read_input(input_path).unwrap();
    let input = lines.inner().into_iter().map(Into::into).collect();
    match part {
        DayPart::One => executor.part_1(input).to_string(),
        DayPart::Two => executor.part_2(input).to_string(),
    }
}
