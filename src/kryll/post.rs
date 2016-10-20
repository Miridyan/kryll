use super::*;
use pandoc::*;

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
        Post {
            input: input,
            output: output,
            input_format: InputFormat::Markdown,
            output_format: OutputFormat::Html5,
        }
    }

    pub fn set_format_type(mut self, input_fmt: InputFormat, 
                           output_fmt: OutputFormat) -> Post<'a> {
        self.input_format = input_fmt;
        self.output_format = output_fmt;

        self
    }

    pub fn format(&self) -> Result<(), PandocError> {
        let mut doc = Pandoc::new();

        doc.add_input(self.input);
        doc.set_output(self.output);
        doc.set_input_format(self.input_format.clone());
        doc.set_output_format(self.output_format.clone());
        doc.execute()
    }
}

impl<'a> fmt::Display for Post<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = File::open(self.output);
        let file_string = handle_file_string(file, self.output);

        write!(f, "{}", file_string)
    }
}
