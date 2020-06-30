use clap::Clap;

use crate::command::ICommand;
use crate::Opts;

#[derive(Clap)]
pub struct TestCommand {}

impl ICommand for TestCommand {
    fn execute(&self, _: &Opts) {
        println!("Hello!")
    }
}
