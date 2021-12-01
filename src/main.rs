mod days;
mod utils;

fn main() {
	let day: u32 = get_day_selection();
	let mut res: (Option<String>, Option<String>) = (None, None);
	let input: &str = &format!("./input/2021-{}.txt", day)[..];
	let lines = utils::read_file_to_lines(input);

	match day {
		1 => { res = days::one::run(lines); }
		2 => { days::two::run(); }
		20201 => { days::twentytwenty_one::run() }
		_ => { println!("Day not started."); }
	}

	println!("Part 1: {}", res.0.unwrap_or(String::from("Not completed.")));
	println!("Part 2: {}", res.1.unwrap_or(String::from("Not completed.")));
}

fn get_day_selection() -> u32 {
	let mut day = String::new();
	println!("Day: ");
	let _ = std::io::stdin().read_line(&mut day).unwrap();
	utils::remove_whitespace(&mut day);
	day.parse::<u32>().expect("Invalid selection.")
}