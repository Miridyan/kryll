extern crate pandoc;
extern crate strfmt;

use std::io;
use std::fs::File;
use std::io::prelude::*;

pub mod page;
pub use self::page::Page;

pub mod post;
pub use self::post::Post;

//pub mod config;
//pub use self::config::*;

pub fn handle_file_string(file: Result<File, io::Error>, path: &str) -> String {
    let mut file_string = String::new();

    if file.is_err() {
        panic!("Could not find {}!", path);
    }

    file.unwrap().read_to_string(&mut file_string);
    file_string
}
