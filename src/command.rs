mod add;
mod list;
mod migrate;
mod serve;

use add::AddCommand;
use clap::Clap;
use list::ListCommand;
use migrate::MigrateCommand;
use serve::ServeCommand;

use crate::Opts;

#[derive(Clap)]
pub enum Command {
    Add(AddCommand),
    List(ListCommand),
    Migrate(MigrateCommand),
    Serve(ServeCommand),
}

pub trait ICommand {
    fn execute(&self, opts: &Opts);
}

pub fn execute(opts: Opts) {
    match &opts.command {
        Command::List(command) => command.execute(&opts),
        Command::Add(command) => command.execute(&opts),
        Command::Migrate(command) => command.execute(&opts),
        Command::Serve(command) => command.execute(&opts),
    }
}
