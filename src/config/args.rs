// В файле src/config/args.rs

use clap::{App, Arg};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("My XML Parser")
            .version("1.0")
            .author("Author Alex Nodus alexnodus@gmail.com>")
            .about("Processes an XML file")
            .arg(Arg::with_name("filename")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .get_matches();

        let filename = matches.value_of("filename").unwrap().to_string();

        Config { filename }
    }
}
