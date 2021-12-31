use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

use structopt::StructOpt;

use opts::Opt;

use crate::ports::cli::structopt::day_part::DayPart;
use crate::ports::cli::structopt::days::{
    run_day_0, run_day_1, run_day_2, run_day_3, run_day_4, run_day_5, run_day_6, run_day_7,
};

mod day_part;
mod days;
mod error;
mod inputs;
mod opts;
mod outputs;

pub fn run() {
    let args: Opt = Opt::from_args();
    run_solution(args.input(), *args.day(), args.part().clone());
}

fn run_solution(input_path: &Path, day: u8, part: DayPart) {
    let output_string: String = match day {
        0 => run_day_0(part, input_path),
        1 => run_day_1(part, input_path),
        2 => run_day_2(part, input_path),
        3 => run_day_3(part, input_path),
        4 => run_day_4(part, input_path),
        5 => run_day_5(part, input_path),
        6 => run_day_6(part, input_path),
        7 => run_day_7(part, input_path),
        _ => unimplemented!(),
    };

    println!("{}", output_string);
}

fn read_input<E: Error, I: TryFrom<String, Error = E>>(input_path: &Path) -> Result<I, E> {
    I::try_from(read_to_string(input_path).unwrap())
}

fn read_input_str<E: Error, I: FromStr<Err = E>>(input_path: &Path) -> Result<I, E> {
    I::from_str(read_to_string(input_path).unwrap().as_str())
}
