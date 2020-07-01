use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contacts {
    twitter: Option<String>,
    github: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    name: String,
    contacts: Contacts,
}

impl Member {
    pub fn new(name: String) -> Self {
        Member {
            name,
            contacts: Contacts {
                twitter: None,
                github: None,
            },
        }
    }
}
