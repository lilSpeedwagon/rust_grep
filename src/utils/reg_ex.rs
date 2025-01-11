use regex::Regex;


pub fn to_case_insensitive(pattern: &Regex) -> Regex {
    return Regex::new((String::from("(?i)") + pattern.as_str()).as_str()).unwrap();
}
