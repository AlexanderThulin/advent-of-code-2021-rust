use crate::utils;

pub fn run() {
	println!("2020 day 1");
	let input = utils::parse_file_to_string("./input/20201.txt");
	let lines: Vec<&str> = input.lines().collect();
	println!("{:?}", lines);
	
	let numbers: Vec<u32> = lines.into_iter().map(utils::parse_str_to_u32).collect();

	let mut result: u32 = 0;

	for x in &numbers {
		for y in &numbers {
			if x + y == 2020 {
				result = x * y;
			}
		}
	}
	
	println!("result: {}", result);
}