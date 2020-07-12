use std::fs::File;
use std::io::{BufWriter, Read, Write};

pub(crate) fn open(path: &str) -> String {
    let mut file = File::open(path).expect("Failed to open the file.");
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("Failed to read from the file.");

    buffer
}

pub(crate) fn save(path: &str, content: String) {
    let file = File::create(path).expect("Failed to open the file.");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", content).expect("Failed to write.");
}
