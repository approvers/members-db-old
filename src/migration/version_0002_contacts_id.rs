use std::env;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_yaml::Error;
use uuid::Uuid;

use super::version_0001_uuid as old;

#[readonly::make]
#[derive(Clone, Deserialize, Serialize)]
pub struct Contacts {
    pub twitter: Option<u64>,
    pub github: Option<u64>,
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

#[derive(Deserialize)]
struct TwitterUser {
    id: u64,
    screen_name: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
}

impl From<old::Contacts> for Contacts {
    fn from(before: old::Contacts) -> Self {
        Contacts {
            twitter: before.twitter.clone().and_then(get_twitter_id),
            github: before.github.clone().and_then(get_github_id),
        }
    }
}

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
            version: 2,
            members: before
                .members
                .clone()
                .into_iter()
                .map(|m| m.into())
                .collect(),
        }
    }
}

pub fn get_twitter_id(screen_name: String) -> Option<u64> {
    let url = format!(
        "https://api.twitter.com/1.1/users/show.json?screen_name={}",
        screen_name
    );

    let token = env::var("TWITTER_TOKEN").expect("No Twitter token found.");
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(url.as_str())
        .header(USER_AGENT, "members-db-rust")
        .header(AUTHORIZATION, format!("Bearer {}", token));

    println!("Fetching from Twitter '{}'.", url);

    let user: TwitterUser = request
        .send()
        .unwrap_or_else(|_| panic!("Failed to fetch Twitter user '{}'.", screen_name))
        .json()
        .expect("Failed to deserialize as JSON.");

    assert_eq!(user.screen_name, screen_name, "Failed to verify user.");

    Some(user.id)
}

pub fn get_github_id(screen_name: String) -> Option<u64> {
    let url = format!("https://api.github.com/users/{}", screen_name);
    let token = env::var("GITHUB_TOKEN").expect("No GitHub token found.");
    let client = reqwest::blocking::Client::new();
    let request = client
        .get(url.as_str())
        .header(USER_AGENT, "members-db-rust")
        .header(AUTHORIZATION, format!("token {}", token));

    println!("Fetching from GitHub '{}'.", url);

    let user: GitHubUser = request
        .send()
        .unwrap_or_else(|_| panic!("Failed to fetch GitHub user '{}'.", screen_name))
        .json()
        .expect("Failed to deserialize as JSON.");

    assert_eq!(user.login, screen_name, "Failed to verify user.");

    Some(user.id)
}

pub fn up(yaml: &str) -> Result<String, Error> {
    let before: old::Payload = serde_yaml::from_str(yaml)?;
    let after: Payload = before.into();

    serde_yaml::to_string(&after)
}
