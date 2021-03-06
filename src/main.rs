mod config;
mod parser;

use config::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use parser::*;
use std::{
    env, fmt,
    fs::{self, File},
    io::{self, Write},
    path::Path,
    path::PathBuf,
    process,
    sync::mpsc::channel,
    time::Duration,
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
    WatchDirIncorrect(String),
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
                ProgramError::WatchDirIncorrect(p) => format!("'{}' is not a directory", p),
            }
        )
    }
}

fn run(config: Config) -> Result<(String, String), ProgramError> {
    if config.help {
        help();
        process::exit(0);
    } else if !config.watch.is_empty() {
        watch(&config)?
    } else {
        read_write(&config)?
    }

    Ok((config.input_file, config.output_file))
}

fn read_write(config: &Config) -> Result<(), ProgramError> {
    let input = fs::read_to_string(&config.input_file).map_err(ProgramError::ReadInput)?;
    let html = Parser::new(&input)
        .parse()
        .ok_or(ProgramError::ParseInput)?;

    // Create missing directories in the output path
    let mut output_dir = PathBuf::from(&config.output_file);
    output_dir.pop(); // Remove the filename and extension from the path
    fs::create_dir_all(output_dir).map_err(ProgramError::CreateOutputFile)?;

    let mut output = File::create(&config.output_file).map_err(ProgramError::CreateOutputFile)?;

    if config.prettify {
        write!(&mut output, "{}", html.pretty_print(0)).map_err(ProgramError::WriteOutput)?;
    } else {
        write!(&mut output, "{}", html).map_err(ProgramError::WriteOutput)?;
    }
    Ok(())
}

fn watch(config: &Config) -> Result<(), ProgramError> {
    if !Path::new(&config.watch).is_dir() {
        return Err(ProgramError::WatchDirIncorrect(config.watch.clone()));
    }

    let (transmit, receive) = channel();
    let mut watcher = watcher(transmit, Duration::from_millis(250)).unwrap(); // Create watcher with 1 second debounce time

    watcher
        .watch(&config.watch, RecursiveMode::Recursive)
        .unwrap(); // Watch for file system events in all subfolders of the specified directory
    println!(
        "\x1b[94;1mInfo:\x1b[0m Watching for changes in {}...",
        &config.watch
    );

    loop {
        match receive.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(changed_file_path) = event {
                    // Match if the event is a file write event
                    let extension = changed_file_path.extension();
                    if let Some(extension) = extension {
                        // Match if the file has an extension
                        if extension == "htmlisp" {
                            // Construct output path
                            let watch_dir = PathBuf::from(&config.watch);
                            let mut output_path = PathBuf::from("output/");

                            let absolute_watch_dir =
                                watch_dir.canonicalize().map_err(ProgramError::ReadInput)?;
                            let absolute_changed_file_path = changed_file_path
                                .canonicalize()
                                .map_err(ProgramError::ReadInput)?;
                            let relative_changed_file_path = absolute_changed_file_path
                                .strip_prefix(absolute_watch_dir)
                                .expect("Couldn't calculate output path");

                            output_path.push(relative_changed_file_path); // (Relative means relative to the watch directory)
                            output_path.set_extension("html");

                            // Create new config
                            match Config::new(&mut env::args()) {
                                Ok(config) => {
                                    let mut config = config; // Make value from match mutable
                                    config.input_file =
                                        changed_file_path.to_str().unwrap().to_string();
                                    config.output_file = output_path.to_str().unwrap().to_string();
                                    println!(
                                        "\x1b[94;1mInfo:\x1b[0m Re-compiling due to changes..."
                                    );

                                    // Parse changed file with new config
                                    match read_write(&config) {
                                        Ok(()) => println!(
                                            "\x1b[32;1mSuccess:\x1b[0m {} -> {}",
                                            relative_changed_file_path.to_string_lossy(),
                                            &config.output_file
                                        ),
                                        // Handle error here instead of propagating it so that the loop keeps running
                                        Err(err) => eprintln!(
                                            "\x1b[31;1mError:\x1b[0m {}: {}",
                                            err,
                                            relative_changed_file_path.to_string_lossy()
                                        ),
                                    }
                                }
                                Err(err) => {
                                    eprintln!("\x1b[31;1mError:\x1b[0m {}", err);
                                    process::exit(1);
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("\x1b[31;1mError:\x1b[0m Watch error: {:?}", err);
                process::exit(1);
            }
        }
    }
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
    -w/--watch <directory> Watch a directory for changes and re-compile:
        outputs to <working directory>/output/,
        preserves input directory structure,
        and makes the -i/--input and -o/--output flags optional

Note:
    If the output file already exists, it will be overwritten
    and if it does not exist, it will be created"#
    );
}
