use std::path::PathBuf;

use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "Advent of Code 2021")]
pub(crate) struct Opt {
    /// Input sample file
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}
