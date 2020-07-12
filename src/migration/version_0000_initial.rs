use serde::Deserialize;

#[readonly::make]
#[derive(Clone, Deserialize)]
pub struct Contacts {
    pub twitter: Option<String>,
    pub github: Option<String>,
}

#[readonly::make]
#[derive(Deserialize)]
pub struct Member {
    pub name: String,
    pub contacts: Contacts,
}
