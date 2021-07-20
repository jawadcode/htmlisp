use std::{env, fs, path::Path, process};

mod parser;

struct Config {
    input_file: String,
    output_file: String,
}

impl Config {
    fn from_args(args: &mut env::Args) -> Option<Self> {
        args.next()?;
        Some(Self {
            input_file: args.next()?,
            output_file: args.next()?,
        })
    }
}

fn main() {
    let mut args = env::args();
    if let Some(config) = Config::from_args(&mut args) {
        let input = fs::read_to_string(&config.input_file).expect("Input file does not exist");
        let html = parser::parse_htmlisp(&input);
        fs::write(&config.output_file, html).unwrap();
        println!(
            "Successfully compiled {} to {}",
            config.input_file, config.output_file
        );
    } else {
        help();
    }
}

fn help() {
    println!(
        r#"HTMLisp

Description:
This program takes in a file of HTML written in Lisp syntax,
parses it and outputs normal HTML

Usage:
htmlisp <input file> <output file>

Note:
If the output file already exists, it will be overwritten
and if it does not exist, it will be created
"#
    );
    process::exit(1);
}
