pub fn main() {
	let directions: Vec<(&str, u32)> = include_str!("input.txt")
		.lines()
		.map(|line| { 
			let line_split: Vec<&str> = line.split(" ").collect();
			(line_split[0], line_split[1].parse::<u32>().expect("Could not parse directions."))
		})
		.collect();

	// PART ONE
	let mut horizontal = 0;
	let mut depth = 0;
	for direction in &directions {
		match direction.0 {
			"forward" => { horizontal += direction.1; }
			"down" => { depth += direction.1 }
			"up" => { depth -= direction.1 }
			_ => { println!("Direction '{:?}' not understood.", direction) }
		}
	}
	let res_one = horizontal * depth;

	// PART TWO
	let mut horizontal = 0;
	let mut depth = 0;
	let mut aim = 0;
	for direction in &directions {
		match direction.0 {
			"forward" => { horizontal += direction.1; depth += aim * direction.1 }
			"down" => { aim += direction.1 }
			"up" => { aim -= direction.1 }
			_ => { println!("Direction '{:?}' not understood.", direction) }
		}
	}
	let res_two = horizontal * depth;

	println!("Part one: {}", res_one); 
	println!("Part two: {}", res_two);
}