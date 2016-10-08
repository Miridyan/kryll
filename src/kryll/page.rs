use super::*;

use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Page {
    pub contents: HashMap<String, String>,
    pub template: String,
}

impl Page {
    pub fn new() -> Page {
        Page {
            contents: HashMap::new(),
            template: String::new(),
        }
    }

    pub fn content(mut self, path: &str, name: &str) -> Page {
        let file = File::open(path);
        let file_string = handle_file_string(file, path);

        self.contents.insert(name.to_string(), file_string);
        self
    }

    pub fn finalize(mut self, path: &str) -> Page {
        let file = File::open(path);
        let file_string = handle_file_string(file, path);

        self.template = file_string;
        self
    }
}
