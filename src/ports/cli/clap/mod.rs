use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

use clap::Parser;

use opts::Opt;

use crate::ports::cli::clap::day_part::DayPart;
use crate::ports::cli::clap::days::{
    run_day_00, run_day_01, run_day_02, run_day_03, run_day_04, run_day_05, run_day_06, run_day_07,
    run_day_08, run_day_09, run_day_10,
};

mod day_part;
mod days;
mod error;
mod inputs;
mod opts;
mod outputs;

pub fn run() {
    let args: Opt = Opt::parse();
    run_solution(args.input(), *args.day(), args.part().clone());
}

fn run_solution(input_path: &Path, day: u8, part: DayPart) {
    let output_string: String = match day {
        0 => run_day_00(part, input_path),
        1 => run_day_01(part, input_path),
        2 => run_day_02(part, input_path),
        3 => run_day_03(part, input_path),
        4 => run_day_04(part, input_path),
        5 => run_day_05(part, input_path),
        6 => run_day_06(part, input_path),
        7 => run_day_07(part, input_path),
        8 => run_day_08(part, input_path),
        9 => run_day_09(part, input_path),
        10 => run_day_10(part, input_path),
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
