use clap::Clap;

use crate::command::ICommand;
use crate::migration::migrate;
use crate::Opts;

#[derive(Clap)]
pub struct MigrateCommand {}

impl ICommand for MigrateCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);

        migrate(&path).expect("Failed to migrate.")
    }
}
