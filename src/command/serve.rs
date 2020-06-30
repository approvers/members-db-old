use clap::Clap;

use crate::command::ICommand;
use crate::database::Database;
use crate::server::Server;
use crate::Opts;

#[derive(Clap)]
pub struct ServeCommand {}

impl ICommand for ServeCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);
        let database = Database::new(path);
        let server = Server::new(database);

        server.start()
    }
}
