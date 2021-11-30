use std::fs;

pub fn remove_whitespace(s: &mut String) {
	s.retain(|c| !c.is_whitespace());
}

pub fn parse_file_to_string(filename: &str) -> String {
	println!("Trying to read file: '{}'", filename);
	fs::read_to_string(filename).expect("Something went wrong when reading the file.")
}

pub fn parse_str_to_u32(text: &str) -> u32 {
	text.parse::<u32>().unwrap()
}