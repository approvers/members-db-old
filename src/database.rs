pub(crate) mod member;

use member::Member;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

fn open(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open the file.");
    let mut string = String::new();

    file.read_to_string(&mut string)
        .expect("Failed to read from the file.");

    string
}

#[derive(Debug)]
pub struct Database {
    path: String,
    members: Vec<Member>,
}

impl Database {
    pub fn new(path: String) -> Self {
        let yaml = open(path.as_str());
        let members: Vec<Member> =
            serde_yaml::from_str(yaml.as_str()).expect("Failed to deserialize.");

        Database { path, members }
    }

    pub fn save(&self) {
        let yaml = serde_yaml::to_string(&self.members).expect("Failed to serialize.");
        let file = File::create(&self.path).expect("Failed to open the file.");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}", yaml).expect("Failed to write.");
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member)
    }
}
