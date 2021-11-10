use opts::Opt;
use structopt::StructOpt;

mod opts;

pub fn run() {
    Opt::from_args();
}
