use structopt::StructOpt;

use opts::Opt;

mod opts;

pub fn run() {
    let _args: Opt = Opt::from_args();
}
