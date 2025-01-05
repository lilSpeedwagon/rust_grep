use crate::utils;

pub struct ReadResult {
    pub line_content: String,
    pub line_number: u32,
}

pub trait TextReader {
    fn read_next(&mut self, pattern: &regex::Regex) -> utils::types::OptionalResult<ReadResult, String>;
}
