use std::io;

extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use offregisters_lib::run_from_path;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}

fn main() -> Result<(), io::Error> {
    let options = Opt::from_args();

    println!("Running from config...");
    let result = run_from_path(options.config);

    println!("Finished...");

    result
}
