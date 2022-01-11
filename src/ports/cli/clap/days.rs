use std::path::Path;

use crate::domain::solution_executor::day_01::Day1SolutionExecutor;
use crate::domain::solution_executor::day_02::Day2SolutionExecutor;
use crate::domain::solution_executor::day_03::Day3SolutionExecutor;
use crate::domain::solution_executor::day_04::Day4SolutionExecutor;
use crate::domain::solution_executor::day_05::Day5SolutionExecutor;
use crate::domain::solution_executor::day_06::Day6SolutionExecutor;
use crate::domain::solution_executor::day_07::Day7SolutionExecutor;
use crate::domain::solution_executor::day_08::Day8SolutionExecutor;
use crate::domain::solution_executor::day_09::Day9SolutionExecutor;
use crate::domain::solution_executor::day_10::Day10SolutionExecutor;
use crate::domain::solution_executor::day_11::Day11SolutionExecutor;
use crate::domain::solution_executor::day_12::Day12SolutionExecutor;
use crate::domain::solution_executor::day_13::Day13SolutionExecutor;
use crate::domain::solution_executor::SolutionExecutor;
use crate::ports::cli::clap::day_part::DayPart;
use crate::ports::cli::clap::inputs::bingo_game::BingoGame;
use crate::ports::cli::clap::inputs::cave_connection::CaveConnection;
use crate::ports::cli::clap::inputs::comma_separated_list::CommaSeparatedList;
use crate::ports::cli::clap::inputs::direction_and_size::DirectionAndSize;
use crate::ports::cli::clap::inputs::lines::Lines;
use crate::ports::cli::clap::inputs::origami_instructions::OrigamiInstructions;
use crate::ports::cli::clap::inputs::straight_line::StraightLine;
use crate::ports::cli::clap::inputs::submarine_display_signals::SubmarineDisplaySignal;
use crate::ports::cli::clap::{read_digit_lines, read_input, read_input_str};

pub(crate) fn run_day_01(part: DayPart, input_path: &Path) -> String {
    let executor = Day1SolutionExecutor::new();
    let input: Lines<u64> = read_input_str(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_02(part: DayPart, input_path: &Path) -> String {
    let executor = Day2SolutionExecutor::new();
    let input: Lines<DirectionAndSize> = read_input(input_path).unwrap();
    let domain_input: Vec<crate::domain::solution_executor::day_02::DirectionAndSize> = input
        .inner()
        .into_iter()
        .map(DirectionAndSize::into)
        .collect();
    match part {
        DayPart::One => executor.part_1(domain_input).to_string(),
        DayPart::Two => executor.part_2(domain_input).to_string(),
    }
}

pub(crate) fn run_day_03(part: DayPart, input_path: &Path) -> String {
    let executor = Day3SolutionExecutor::new();
    let input: Lines<String> = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_04(part: DayPart, input_path: &Path) -> String {
    let executor = Day4SolutionExecutor::new();
    let input: BingoGame = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.into()).to_string(),
        DayPart::Two => executor.part_2(input.into()).to_string(),
    }
}

pub(crate) fn run_day_05(part: DayPart, input_path: &Path) -> String {
    let executor = Day5SolutionExecutor::new();
    let input: Lines<StraightLine> = read_input(input_path).unwrap();
    let domain_input: Vec<crate::domain::solution_executor::day_05::StraightLine> =
        input.inner().into_iter().map(Into::into).collect();
    match part {
        DayPart::One => executor.part_1(domain_input).to_string(),
        DayPart::Two => executor.part_2(domain_input).to_string(),
    }
}

pub(crate) fn run_day_06(part: DayPart, input_path: &Path) -> String {
    let executor = Day6SolutionExecutor::new();
    let lines: Lines<CommaSeparatedList<u8>> = read_input_str(input_path).unwrap();
    let input = lines.inner().pop().unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_07(part: DayPart, input_path: &Path) -> String {
    let executor = Day7SolutionExecutor::new();
    let lines: Lines<CommaSeparatedList<u64>> = read_input_str(input_path).unwrap();
    let input = lines.inner().pop().unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_08(part: DayPart, input_path: &Path) -> String {
    let executor = Day8SolutionExecutor::new();
    let lines: Lines<SubmarineDisplaySignal> = read_input(input_path).unwrap();
    let input = lines.inner().into_iter().map(Into::into).collect();
    match part {
        DayPart::One => executor.part_1(input).to_string(),
        DayPart::Two => executor.part_2(input).to_string(),
    }
}

pub(crate) fn run_day_09(part: DayPart, input_path: &Path) -> String {
    let executor = Day9SolutionExecutor::new();
    let input = read_digit_lines(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input).to_string(),
        DayPart::Two => executor.part_2(input).to_string(),
    }
}

pub(crate) fn run_day_10(part: DayPart, input_path: &Path) -> String {
    let executor = Day10SolutionExecutor::new();
    let input: Lines<String> = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.inner()).to_string(),
        DayPart::Two => executor.part_2(input.inner()).to_string(),
    }
}

pub(crate) fn run_day_11(part: DayPart, input_path: &Path) -> String {
    let executor = Day11SolutionExecutor::new();
    let input = read_digit_lines(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input).to_string(),
        DayPart::Two => executor.part_2(input).to_string(),
    }
}

pub(crate) fn run_day_12(part: DayPart, input_path: &Path) -> String {
    let executor = Day12SolutionExecutor::new();
    let lines: Lines<CaveConnection> = read_input(input_path).unwrap();
    let input: Vec<(String, String)> = lines
        .inner()
        .into_iter()
        .map(CaveConnection::into)
        .collect();
    match part {
        DayPart::One => executor.part_1(input).to_string(),
        DayPart::Two => executor.part_2(input).to_string(),
    }
}

pub(crate) fn run_day_13(part: DayPart, input_path: &Path) -> String {
    let executor = Day13SolutionExecutor::new();
    let input: OrigamiInstructions = read_input(input_path).unwrap();
    match part {
        DayPart::One => executor.part_1(input.into()).to_string(),
        DayPart::Two => executor.part_2(input.into()).to_string(),
    }
}
