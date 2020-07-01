use rocket::get;
use rocket::routes;
use rocket::State;
use rocket_contrib::json::Json;

use crate::database::member::Member;
use crate::database::Database;

#[get("/")]
fn index(state: State<Database>) -> Json<Vec<Member>> {
    Json(state.get_members().clone())
}

pub struct Server {
    database: Database,
}

impl Server {
    pub fn new(database: Database) -> Self {
        Server { database }
    }

    pub fn start(self) {
        rocket::ignite()
            .mount("/", routes![index])
            .manage(self.database)
            .launch();
    }
}
