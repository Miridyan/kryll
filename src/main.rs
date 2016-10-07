extern crate strfmt;
extern crate pandoc;

use strfmt::strfmt;

use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct Post {
    contents: HashMap<String, String>,
    template: String,
}

impl Post {
    fn new() -> Post {
        Post {
            contents: HashMap::new(),
            template: String::new(),
        }
    }
    
    fn add_content(mut self, path: &str, name: &str) -> Post {
        let file = File::open(path);
        let mut file_string = handle_file_string(file, path);

        self.contents.insert(name.to_string(), file_string);
        self
    }

    fn template(mut self, path: &str) -> Post {
        let file = File::open(path);
        let mut file_string = handle_file_string(file, path);

        self.template = file_string;
        self
    }

    fn build(self) -> Post {
        assert!(!self.template.is_empty(), "You require a template");
        self
    }
}

fn handle_file_string(file: Result<File, io::Error>, path: &str) -> String {
    let mut file_string = String::new();
    
    if file.is_err() {
        panic!("Could not find {}!", path);
    }

    file.unwrap().read_to_string(&mut file_string);
    file_string
}


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for arg in args {
        println!("{}", arg);
    }

    let mut fmt = String::new();
    let mut vars = HashMap::new();
    let mut file = File::open("html/head.html").unwrap();

    vars.insert("PageTitle".to_string(), "This is a test a formatter!");
    file.read_to_string(&mut fmt);

    let fmtd = strfmt(&fmt, &vars).unwrap();
    println!("{}", fmtd);


    let test_post_please_ignore = Post::new()
        .add_content("html/head.html", "head")
        .add_content("html/body.html", "body")
        .add_content("html/header.html", "header")
        .add_content("html/footer.html", "footer")
        .template("html/post.html")
        .build();
}
