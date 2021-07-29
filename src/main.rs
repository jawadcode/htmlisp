mod config;
mod parser;

use config::*;
use parser::*;
use std::{
    env, fmt,
    fs::{self, File},
    io::{self, Write},
    process,
};

fn main() {
    let mut args = env::args();
    match Config::new(&mut args).map(run) {
        Ok(res) => match res {
            Ok((input_file, output_file)) => {
                println!(
                    "\x1b[32;1mSuccess:\x1b[0m {} -> {}",
                    input_file, output_file
                );
            }
            Err(err) => {
                eprintln!("\x1b[31;1mError:\x1b[0m {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("\x1b[31;1mError:\x1b[0m {}", err);
            process::exit(1);
        }
    };
}

enum ProgramError {
    ReadInput(io::Error),
    ParseInput,
    CreateOutputFile(io::Error),
    WriteOutput(io::Error),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgramError::ReadInput(e) =>
                    format!("Failed to read input file\n({})", e.to_string()),
                ProgramError::ParseInput => "Failed to parse input file".to_string(),
                ProgramError::CreateOutputFile(e) =>
                    format!("Failed to create output file\n({})", e.to_string()),
                ProgramError::WriteOutput(e) =>
                    format!("Failed to write to output file:\n({})", e.to_string()),
            }
        )
    }
}

fn run(config: Config) -> Result<(String, String), ProgramError> {
    if config.help {
        help();
        process::exit(0);
    } else if config.watch {
        watch(&config)?
    } else {
        read_write(&config)?
    }

    Ok((config.input_file, config.output_file))
}

fn read_write(config: &Config) -> Result<(), ProgramError> {
    let input = fs::read_to_string(&config.input_file).map_err(|e| ProgramError::ReadInput(e))?;
    let html = Parser::new(&input)
        .parse()
        .ok_or(ProgramError::ParseInput)?;

    let mut output =
        File::create(&config.output_file).map_err(|e| ProgramError::CreateOutputFile(e))?;

    if config.prettify {
        write!(&mut output, "{}", html.pretty_print(0))
            .map_err(|e| ProgramError::WriteOutput(e))?;
    } else {
        write!(&mut output, "{}", html).map_err(|e| ProgramError::WriteOutput(e))?;
    }
    Ok(())
}

fn watch(config: &Config) -> Result<(), ProgramError> {
    // stuff
    loop {
        // things
    }
    Ok(())
}

fn help() {
    println!(
        r#"HTMLisp

Description:
    This program takes in a file of HTMLisp,
    parses it and outputs normal HTML

Usage:
    htmlisp -i/--input <input file> -o/--output <output file>
    
    Optional Flags:
        -p/--prettify Output prettified HTML


Note:
    If the output file already exists, it will be overwritten
    and if it does not exist, it will be created"#
    );
}
