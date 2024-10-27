use clap::Parser;
use regex::Regex;
// use std::path::PathBuf;
use glob::glob;

use crate::reader::FileReader;

pub mod reader;


#[derive(Parser)]
struct CliArgs {
    /// The pattern to look for
    pattern: Regex,
    /// The path to the file to read
    path: String,
}

fn main() {

    // read cmd args
    let args = CliArgs::parse();

    // iterate over files
    for glob_entry in glob(args.path.as_str()).expect("Failed to read glob") {
        let file_path = glob_entry.expect("Cannot read glob entry");
        let path_str = file_path.to_string_lossy();
        if file_path.is_dir() {
            println!("Skip dir {path_str}");
        }

        let mut reader = FileReader::new(&file_path);
        loop {
            let next_result = reader.read_next(&args.pattern)
                .expect(format!("Error reading {path_str}").as_str());
            match next_result {
                Some(result) => {
                    let line = result.line_content;
                    let line_number = result.line_number;
                    println!("{path_str} ({line_number}): {line}");
                },
                None => {
                    break;
                }
            }
        }
    }
}