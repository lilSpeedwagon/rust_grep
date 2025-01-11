use std::io;
use std::path;
use std::error::Error;

use clap::Parser;
use regex::Regex;
use glob;
use flexi_logger::{Logger, WriteMode};
use log::{debug, info};

pub mod reader;
pub mod utils;

use reader::base::TextReader;

#[derive(Parser)]
struct CliArgs {
    /// The pattern to look for.
    pattern: Regex,
    /// The glob pattern for the files to scan.
    glob: String,
    /// Verbose log level flag.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    /// Ingore letters case flag.
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

fn read_file(file_path: &path::Path, pattern: &Regex) -> Result<(), Box<dyn Error>> {
    let path_str = file_path.to_string_lossy();
    let open_file_reader_result = reader::file_reader::FileReader::new(&file_path);
    match open_file_reader_result {
        Ok(mut reader) => {
            loop {
                let next_result = reader.read_next(&pattern);
                match next_result {
                    utils::types::OptionalResult::Ok(result) => {
                        let line = result.line_content;
                        let line_number = result.line_number;
                        info!("{path_str} ({line_number}): {line}");
                    },
                    utils::types::OptionalResult::Err(err) => {
                        debug!("Warning! Cannot read file {path_str}: {err}");
                        break;
                    },
                    utils::types::OptionalResult::None => {
                        break;
                    }
                }
            }
        },
        Err(err) => return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Cannot open file {path_str}: {err}")))),
    }
    Ok(())
}


fn grep(glob: &String, pattern: &Regex) -> Result<(), Box<dyn Error>> {
    // Iterate over files according to the glob pattern.
    let glob_iterator = glob::glob(glob.as_str())?;
    for glob_entry in glob_iterator {
        let file_path = glob_entry?;
        if file_path.is_dir() {
            continue;
        }

        read_file(file_path.as_path(), &pattern)?;
    }

    Ok(())
}

fn main() {
    // Read cmd args.
    let args = CliArgs::parse();

    // Init logger.
    let log_level = if args.verbose { "debug" } else { "info" };
    Logger::try_with_str(log_level)
        .unwrap()
        .write_mode(WriteMode::Direct)
        .format(|w, _, record| {
            write!(w, "{}", record.args())
        })
        .start()
        .unwrap();

    // Prepare pattern.
    let mut pattern = args.pattern;
    if args.ignore_case {
        pattern = utils::reg_ex::to_case_insensitive(&pattern);
    }

    let grep_result = grep(&args.glob, &pattern);
    match grep_result {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    }
}
