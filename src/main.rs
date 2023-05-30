mod cli;
mod goals;
mod save_file;
mod tasks;
mod utils;
use cli::{apply_cli_operation, Cli};
use structopt::StructOpt;

fn main() {
    apply_cli_operation(Cli::from_args());
}
