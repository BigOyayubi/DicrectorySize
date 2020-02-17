
use std::error;
use std::process;

#[macro_use]
extern crate log;

use args::Args;
mod app;
mod args;
mod logger;
mod calc;

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main(args: Args) -> Result<()> {
    let result = calc::calc_dir_size(&args);

    if let Err(err) = result {
        debug!("{:?}", err);
        process::exit(1);
    }

    let sum = result?;

    for v in sum {
        let (path, size) = v;
        println!("{:?} {}", path, size);
    }
    process::exit(0);
}
