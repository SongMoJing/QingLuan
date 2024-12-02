use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

pub struct FileWrapper {
    works: bool,
    file: Option<BufReader<File>>,
}

impl FileWrapper {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path);
        Self {
            works: file.is_ok(),
            file: {
                Some(BufReader::new({
                    match file {
                        Ok(file) => file,
                        Err(_) => return Self { works: false, file: None },
                    }
                }))
            },
        }
    }

    pub fn works(&mut self) -> bool {
        self.works
    }

    pub fn next(&mut self) -> Option<String> {
        if self.works {
            self.file.as_mut().unwrap().lines().next().map(|line| line.unwrap())
        } else {
            None
        }
    }
}