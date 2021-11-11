use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

use structopt::StructOpt;

use opts::Opt;

use crate::domain::day_0::Day0SolutionExecutor;
use crate::ports::cli::structopt::error::ParseInputError;
use crate::ports::cli::structopt::inputs::Lines;
use crate::ports::cli::structopt::outputs::List;

mod error;
mod inputs;
mod opts;
mod outputs;

pub fn run() {
    let args: Opt = Opt::from_args();
    run_solution(args.input(), *args.day(), *args.part());
}

fn run_solution(input_path: &Path, day: u8, part: u8) {
    match day {
        0 => match part {
            1 => {
                let input: Lines<String> = unwrap_or_panic(read_input(input_path));
                let executor = Day0SolutionExecutor::new();
                let output = List::new(executor.part_1(input.inner()));

                println!("{}", output);
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn read_input<I: TryFrom<String, Error = ParseInputError>>(
    input_path: &Path,
) -> Result<I, ParseInputError> {
    I::try_from(read_to_string(input_path).unwrap())
}

fn unwrap_or_panic<T>(result: Result<T, impl Error>) -> T {
    result.unwrap()
}
