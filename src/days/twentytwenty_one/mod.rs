use crate::utils;

pub fn run() {
	println!("2020 day 1");
	let input = utils::parse_file_to_string("./input/2020-1.txt");
	let lines: Vec<&str> = input.lines().collect();
	println!("{:?}", lines);
	
	let numbers: Vec<u32> = lines.into_iter().map(|line| line.parse::<u32>().unwrap()).collect();

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