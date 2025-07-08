mod cli;
mod utils;

use clap::Parser;
use cli::cli;
use utils::Args;
fn main() {
    let args = Args::parse();

    if !args.dir.is_dir() {
        panic!("Provided directory does not exist.");
    }

    cli(&args);
}
