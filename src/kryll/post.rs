use super::*;
use pandoc::*;

use std::io;
use std::fmt;
use std::fs::File;

#[derive(Debug)]
pub struct Post<'a> {
    pub input: &'a str,
    pub output: &'a str,
    pub input_format: InputFormat,
    pub output_format: OutputFormat,
}

impl<'a> Post<'a> {
    pub fn new(input: &'a str, output: &'a str) -> Post<'a> {
        //let file = File::open(path);
        //let file_string = handle_file_string(file, path);
        
        Post {
            input: input,
            output: output,
            input_format: InputFormat::Markdown,
            output_format: OutputFormat::Html5,
        }
    }

//    pub fn format(mut self) -> Post<'a> {
//        let doc = Pandoc::new()
//            .set_input_format(self.input_format)
//            .set_output_format(self.output_format)
//    }

    pub fn set_format_type(mut self, input_fmt: InputFormat, 
                           output_fmt: OutputFormat) -> Post<'a> {
        self.input_format = input_fmt;
        self.output_format = output_fmt;

        self
    }

//    pub fn out(&self, path: &str) -> Result<(), io::Error> {
//        let mut out_file = try!(File::create(path));
//
//        try!(out_file.write_all(self.output.as_bytes()));
//        println!("Successfully wrote to {}.", path);
//        
//        Ok(())
//    }
}

impl<'a> fmt::Display for Post<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = File::open(self.output);
        let file_string = handle_file_string(file, self.output);

        write!(f, "{}", file_string)
    }
}
