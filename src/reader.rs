// use std::fmt::format;
use std::io::BufRead;
use std::fs;
use std::io;
use regex;


pub struct ReadResult {
    pub line_content: String,
    pub line_number: u32,
}

pub struct FileReader {
    buffer_reader: io::BufReader<fs::File>,
    line_counter: u32,
}

impl FileReader {
    pub fn new(path: &std::path::PathBuf) -> Result<Self, String> {
        let file = match fs::File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Cannot open file {path:?}: {err:?}")),
        };
        let buffer_reader = io::BufReader::new(file);
        return Ok(
            FileReader{
                buffer_reader: buffer_reader,
                line_counter: 0,
            }
        );
    }

    pub fn read_next(&mut self, pattern: &regex::Regex) -> Result<Option<ReadResult>, String> {
        let mut buffer = String::new();
        loop {
            self.line_counter += 1;
            let line_len = match self.buffer_reader.read_line(&mut buffer) {
                Ok(line) => line,
                Err(err) => {
                    return Err(format!("Cannot read file: {err}"));
                }
            };

            if line_len == 0 {
                return Ok(None);
            }

            if pattern.is_match(&buffer) {
                let line = String::from(buffer.strip_suffix("\n").unwrap_or(&buffer));
                return Ok(Some(ReadResult{line_content: line, line_number: self.line_counter}));
            }

            buffer.clear();
        }
    }
}
