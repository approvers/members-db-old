mod add;
mod list;
mod serve;
mod test;

use add::AddCommand;
use clap::Clap;
use list::ListCommand;
use serve::ServeCommand;
use test::TestCommand;

use crate::Opts;

#[derive(Clap)]
pub enum Command {
    Add(AddCommand),
    Test(TestCommand),
    List(ListCommand),
    Serve(ServeCommand),
}

pub trait ICommand {
    fn execute(&self, opts: &Opts);
}

pub fn execute(opts: Opts) {
    match &opts.command {
        Command::Test(command) => command.execute(&opts),
        Command::List(command) => command.execute(&opts),
        Command::Add(command) => command.execute(&opts),
        Command::Serve(command) => command.execute(&opts),
    }
}
