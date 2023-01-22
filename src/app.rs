use std::fs;

use crate::parser;
use crate::args::Config;
use crate::reader::{self, Bookmarks};

pub struct App {
    cf: Config,
    output: String,
}

impl App {
    pub fn new() -> App {
        App {
            cf: Config::new(),
            output: String::new(),
        }
    }

    pub fn run(&mut self) {
        let bm: Bookmarks = reader::read_file(self.cf.input_file.clone()).expect("failed to read file");
        dbg!(&bm);
        self.output = parser::parse_file(&bm);
        // write file to outfile in HTML
        fs::write(&self.cf.output_file, &self.output).expect("Unable to write file");
    }
}
