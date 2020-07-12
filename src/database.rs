pub(crate) mod member;

use member::Member;

use crate::filesystem;

#[derive(Debug)]
pub struct Database {
    path: String,
    members: Vec<Member>,
}

impl Database {
    pub fn new(path: String) -> Self {
        let yaml = filesystem::open(path.as_str());
        let members: Vec<Member> =
            serde_yaml::from_str(yaml.as_str()).expect("Failed to deserialize.");

        Database { path, members }
    }

    pub fn save(&self) {
        let yaml = serde_yaml::to_string(&self.members).expect("Failed to serialize.");

        filesystem::save(&self.path, yaml)
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member)
    }
}
