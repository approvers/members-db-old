use clap::Clap;

use crate::command::ICommand;
use crate::database::member::Member;
use crate::database::Database;
use crate::Opts;

#[derive(Clap)]
pub struct AddCommand {
    name: String,
}

impl ICommand for AddCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);
        let mut database = Database::new(path);

        database.add_member(Member::new(self.name.clone()));
        database.save();
    }
}
