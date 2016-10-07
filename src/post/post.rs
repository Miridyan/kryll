use super::*;

use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Post {
    pub contents: HashMap<String, String>,
    pub template: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            contents: HashMap::new(),
            template: String::new(),
        }
    }

    pub fn content(mut self, path: &str, name: &str) -> Post {
        let file = File::open(path);
        let file_string = handle_file_string(file, path);

        self.contents.insert(name.to_string(), file_string);
        self
    }

    pub fn template(mut self, path: &str) -> Post {
        let file = File::open(path);
        let file_string = handle_file_string(file, path);

        self.template = file_string;
        self
    }

    pub fn build(self) -> Post {
        assert!(!self.template.is_empty(), "You require a template");
        self
    }
}
