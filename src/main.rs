#![feature(proc_macro_hygiene, decl_macro)]

mod command;
mod database;
mod filesystem;
mod server;

use clap::Clap;
use command::Command;

#[derive(Clap)]
pub struct Opts {
    #[clap(short, long, default_value = "database.yaml")]
    database: String,

    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let opts: Opts = Opts::parse();

    command::execute(opts)
}
