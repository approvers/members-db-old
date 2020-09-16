use rocket::get;
use rocket::response::status::NotFound;
use rocket::routes;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

use crate::database::member::Member;
use crate::database::Database;

#[get("/")]
fn index(state: State<Database>) -> Json<Vec<Member>> {
    Json(state.get_members().clone())
}

#[get("/<id>")]
fn show(state: State<Database>, id: Uuid) -> Result<Json<Member>, NotFound<()>> {
    state
        .find_member(uuid::Uuid::from_bytes(*id.as_bytes()))
        .ok_or(NotFound(()))
        .map(|m| Json(m.clone()))
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
            .mount("/", routes![index, show])
            .manage(self.database)
            .launch();
    }
}
