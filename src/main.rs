mod days;
mod utils;

fn main() {
	let day: u32 = get_day_selection();

	match day {
		1 => { days::one::run(); }
		2 => { days::two::run(); }
		20201 => { days::twentytwenty_one::run() }
		_ => { println!("Day not started."); }
	}
}

fn get_day_selection() -> u32 {
	let mut day = String::new();
	println!("Day: ");
	let _ = std::io::stdin().read_line(&mut day).unwrap();
	utils::remove_whitespace(&mut day);
	println!("Selected day {}", day);
	day.parse::<u32>().expect("Invalid selection.")
}