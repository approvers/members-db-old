use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use uuid::Uuid;

use super::version_0000_initial as old;

#[readonly::make]
#[derive(Clone, Deserialize, Serialize)]
pub struct Contacts {
    pub twitter: Option<String>,
    pub github: Option<String>,
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
    fn from(before: old::Contacts) -> Contacts {
        Contacts {
            twitter: before.twitter.clone(),
            github: before.github.clone(),
        }
    }
}

impl From<old::Member> for Member {
    fn from(before: old::Member) -> Member {
        Member {
            id: Uuid::new_v4(),
            name: before.name.clone(),
            contacts: before.contacts.clone().into(),
        }
    }
}

impl From<Vec<old::Member>> for Payload {
    fn from(before: Vec<old::Member>) -> Self {
        Payload {
            version: 1,
            members: before.into_iter().map(|member| member.into()).collect(),
        }
    }
}

pub fn up(yaml: &str) -> Result<String, Error> {
    let before: Vec<old::Member> = serde_yaml::from_str(yaml)?;
    let after: Payload = before.into();

    serde_yaml::to_string(&after)
}
