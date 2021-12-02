use std::fs;

pub fn remove_whitespace(s: &mut String) {
	s.retain(|c| !c.is_whitespace());
}

pub fn parse_file_to_string(filename: &str) -> String {
	fs::read_to_string(filename).expect(&format!("Something went wrong when reading the file {}.", filename)[..])
}