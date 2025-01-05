#[cfg(test)]
mod file_reader_tests {
    use std::io::Write;
    use std::path;

    use regex::Regex;
    use tempfile::NamedTempFile;

    use crate::reader::{base::TextReader, base::ReadResult, file_reader::FileReader};
    use crate::utils::types::OptionalResult;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(file, "{}", content).expect("Cannot write temp file");
        file
    }

    #[test]
    fn test_read_empty() {
        let file = create_temp_file("");

        let mut reader = FileReader::new(file.path()).unwrap();
        let read_result = reader.read_next(&Regex::new(".").unwrap());

        assert!(matches!(read_result, OptionalResult::None));
    }

    #[test]
    fn test_read_single_line_match() {
        let file = create_temp_file("Hello, Rust!");

        let mut reader = FileReader::new(file.path()).unwrap();
        let read_result = reader.read_next(&Regex::new("Rust").unwrap());

        assert!(matches!(read_result, OptionalResult::Ok(ReadResult { .. })));
        if let OptionalResult::Ok(result) = read_result {
            assert_eq!(result.line_content, "Hello, Rust!");
            assert_eq!(result.line_number, 1);
        }
    }

    #[test]
    fn test_read_multiple_matches() {
        let file = create_temp_file("Rust\nIs\nCool");

        let mut reader = FileReader::new(file.path()).unwrap();
        let regex = Regex::new(".*").unwrap();

        // First match
        let result1 = reader.read_next(&regex);
        assert!(matches!(result1, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result1 {
            assert_eq!(read_result.line_content, "Rust");
            assert_eq!(read_result.line_number, 1);
        }

        // Second match
        let result2 = reader.read_next(&regex);
        assert!(matches!(result2, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result2 {
            assert_eq!(read_result.line_content, "Is");
            assert_eq!(read_result.line_number, 2);
        }

        // Third match
        let result3 = reader.read_next(&regex);
        assert!(matches!(result3, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result3 {
            assert_eq!(read_result.line_content, "Cool");
            assert_eq!(read_result.line_number, 3);
        }

        // No more matches
        let result4 = reader.read_next(&regex);
        assert!(matches!(result4, OptionalResult::None));
    }

    #[test]
    fn test_read_no_match() {
        let file = create_temp_file("No patterns here!");

        let mut reader = FileReader::new(file.path()).unwrap();
        let read_result = reader.read_next(&Regex::new("Rust").unwrap());

        assert!(matches!(read_result, OptionalResult::None));
    }

    #[test]
    fn test_read_with_special_characters() {
        let file = create_temp_file("Hello, [Rust]!\nSpecial (characters)?");

        let mut reader = FileReader::new(file.path()).unwrap();
        let regex = Regex::new(r"\[Rust\]").unwrap();

        // Match for `[Rust]`
        let result = reader.read_next(&regex);
        assert!(matches!(result, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result {
            assert_eq!(read_result.line_content, "Hello, [Rust]!");
            assert_eq!(read_result.line_number, 1);
        }

        // No more matches
        let result = reader.read_next(&regex);
        assert!(matches!(result, OptionalResult::None));
    }

    #[test]
    fn test_read_error_handling() {
        // Non-existent file path.
        let path = path::Path::new("dummy_file.fake.txt");
        let result = FileReader::new(&path);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.starts_with("Cannot open file \"dummy_file.fake.txt\""));
        }
    }

    #[test]
    fn test_read_partial_matches() {
        let file = create_temp_file("Rust is fun\nRustaceans rule\nLove Rust!");

        let mut reader = FileReader::new(file.path()).unwrap();
        let regex = Regex::new("Rust").unwrap();

        // First match
        let result1 = reader.read_next(&regex);
        assert!(matches!(result1, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result1 {
            assert_eq!(read_result.line_content, "Rust is fun");
            assert_eq!(read_result.line_number, 1);
        }

        // Second match
        let result2 = reader.read_next(&regex);
        assert!(matches!(result2, OptionalResult::Ok(_)));
        if let OptionalResult::Ok(read_result) = result2 {
            assert_eq!(read_result.line_content, "Rustaceans rule");
            assert_eq!(read_result.line_number, 2);
        }
    }
}
