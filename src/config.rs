use std::{env, fmt};

pub struct Config {
    pub help: bool,
    pub prettify: bool,
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self, ArgsError> {
        args.next();

        let mut cfg = Config {
            help: false,
            prettify: false,
            input_file: String::new(),
            output_file: String::new(),
        };
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" | "--input" => {
                    cfg.input_file = args.next().ok_or(ArgsError::InputMissing)?;
                }
                "-o" | "--output" => {
                    cfg.output_file = args.next().ok_or(ArgsError::OutputMissing)?;
                }
                "-p" | "--prettify" => {
                    cfg.prettify = true;
                }
                "-h" | "--help" => {
                    cfg.help = true;
                }
                unknown => return Err(ArgsError::UnknownArg(unknown.to_string())),
            }
        }

        if cfg.input_file.is_empty() && !cfg.help {
            return Err(ArgsError::InputMissing);
        } else if cfg.output_file.is_empty() && !cfg.help {
            return Err(ArgsError::OutputMissing);
        }

        Ok(cfg)
    }
}

pub enum ArgsError {
    InputMissing,
    OutputMissing,
    UnknownArg(String),
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgsError::InputMissing => "Input file not specified".to_string(),
                ArgsError::OutputMissing => "Output file not specified".to_string(),
                ArgsError::UnknownArg(s) => format!("Unknown flag '{}'", s),
            }
        )
    }
}