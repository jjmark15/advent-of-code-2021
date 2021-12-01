use std::path::PathBuf;

use crate::ports::cli::structopt::day_part::DayPart;
use structopt::StructOpt;

/// Executor of the 2021 Advent of Code challenge solutions
#[derive(StructOpt, Debug, derive_getters::Getters)]
#[structopt(name = "Advent of Code 2021")]
pub(crate) struct Opt {
    /// Input sample file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Challenge day
    #[structopt(short, long)]
    day: u8,

    /// Challenge part
    #[structopt(short, long, default_value = "1")]
    part: DayPart,
}
