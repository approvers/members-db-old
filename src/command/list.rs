use clap::Clap;

use crate::command::ICommand;
use crate::database::Database;
use crate::Opts;

#[derive(Clap)]
pub struct ListCommand {}

impl ICommand for ListCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);
        let database = Database::new(path);

        for member in database.members {
            println!("{:?}", member);
        }
    }
}
