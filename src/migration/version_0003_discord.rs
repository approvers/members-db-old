use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use uuid::Uuid;

use super::version_0002_contacts_id as old;

#[readonly::make]
#[derive(Clone, Deserialize, Serialize)]
pub struct Contacts {
    pub twitter: Option<u64>,
    pub github: Option<u64>,
    pub discord: Option<u64>,
}

#[readonly::make]
#[derive(Clone, Deserialize, Serialize)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub contacts: Contacts,
}

#[readonly::make]
#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub version: u32,
    pub members: Vec<Member>,
}

impl From<old::Contacts> for Contacts {
    fn from(before: old::Contacts) -> Self {
        Contacts {
            twitter: before.twitter,
            github: before.github,
            discord: None,
        }
    }
}

// noinspection DuplicatedCode
impl From<old::Member> for Member {
    fn from(before: old::Member) -> Self {
        Member {
            id: before.id,
            name: before.name.clone(),
            contacts: before.contacts.clone().into(),
        }
    }
}

impl From<old::Payload> for Payload {
    fn from(before: old::Payload) -> Self {
        Payload {
            version: 3,
            members: before
                .members
                .clone()
                .into_iter()
                .map(|m| m.into())
                .collect(),
        }
    }
}

pub fn up(yaml: &str) -> Result<String, Error> {
    let before: old::Payload = serde_yaml::from_str(yaml)?;
    let after: Payload = before.into();

    serde_yaml::to_string(&after)
}
