use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contacts {
    twitter: Option<u64>,
    github: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    id: Uuid,
    name: String,
    contacts: Contacts,
}

impl Member {
    pub fn new(name: String) -> Self {
        Member {
            id: Uuid::new_v4(),
            name,
            contacts: Contacts {
                twitter: None,
                github: None,
            },
        }
    }
}
