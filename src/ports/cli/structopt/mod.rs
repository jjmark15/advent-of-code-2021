use std::fs::read_to_string;
use std::path::Path;

use structopt::StructOpt;

use opts::Opt;

use crate::ports::cli::structopt::days::{run_day_0, run_day_1};
use crate::ports::cli::structopt::error::ParseInputError;

mod days;
mod error;
mod inputs;
mod opts;
mod outputs;

pub fn run() {
    let args: Opt = Opt::from_args();
    run_solution(args.input(), *args.day(), *args.part());
}

fn run_solution(input_path: &Path, day: u8, part: u8) {
    let output_string: String = match day {
        0 => run_day_0(part, input_path),
        1 => run_day_1(part, input_path),
        _ => unimplemented!(),
    };

    println!("{}", output_string);
}

fn read_input<I: TryFrom<String, Error = ParseInputError>>(
    input_path: &Path,
) -> Result<I, ParseInputError> {
    I::try_from(read_to_string(input_path).unwrap())
}
