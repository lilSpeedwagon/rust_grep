use std::path::PathBuf;

use clap::Parser;
use regex::Regex;
use glob::glob;
use flexi_logger::{Logger, WriteMode};
use log::{debug, info};

use crate::reader::FileReader;

pub mod reader;
pub mod utils;


#[derive(Parser)]
struct CliArgs {
    /// The pattern to look for.
    pattern: Regex,
    /// The glob pattern for the files to scan.
    glob: String,
    /// Verbose log level flag.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn read_file(file_path: PathBuf, pattern: &Regex) {
    let path_str = file_path.to_string_lossy();
    let open_file_reader_result = FileReader::new(&file_path);
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
        Err(err) => {
            debug!("Warning! Cannot open file {path_str}: {err}");
        }
    }
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

    // Iterate over files according to the glob pattern.
    let read_glob_result = glob(args.glob.as_str());
    match read_glob_result {
        Ok(glob_iterator) => {
            for glob_entry in glob_iterator {
                let file_path = glob_entry.expect("Cannot read glob entry");
                if file_path.is_dir() {
                    continue;
                }

                read_file(file_path, &args.pattern);
            }
        },
        Err(err) => panic!("Invalid glob: {err}")
    }
}
