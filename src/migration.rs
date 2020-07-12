mod version_0000_initial;
mod version_0001_uuid;

use serde::Deserialize;
use serde_yaml::Error;

use crate::filesystem;

#[derive(Deserialize)]
struct Outline {
    version: Option<usize>,
}

pub fn migrate(path: &str) -> Result<(), Error> {
    let mut yaml = filesystem::open(path);

    loop {
        let outline: Result<Outline, Error> = serde_yaml::from_str(&yaml);
        let version = outline.map_or(None, |o| o.version).unwrap_or(0);

        yaml = match version {
            0 => version_0001_uuid::up(&yaml)?,
            _ => break,
        };

        println!("Migrated from {}", version)
    }

    filesystem::save(path, yaml);

    Ok(())
}
