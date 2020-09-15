use std::{
    fs::File,
    io::{prelude::*, Error},
};

pub struct WriteFile {
    location: String,
    file: File,
}

impl WriteFile {
    pub fn new(location: String) -> Self {
        let mut file = File::create(&location).unwrap();

        WriteFile { location, file }
    }

    pub fn write_string(&mut self, contents: &str) {
        self.file.write_all(contents.as_bytes()).unwrap();
    }

    pub fn write_all(&mut self, contents: &[u8]) {
        self.file.write_all(&contents).unwrap();
    }
}

pub fn read_file(location: &String) -> Result<String, Error> {
    let mut file = File::open(&location)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
