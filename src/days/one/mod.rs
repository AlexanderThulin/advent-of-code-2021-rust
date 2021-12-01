pub fn run() -> (Option<String>, Option<String>) {
	let depths = include_str!("input.txt")
		.lines()
		.map(|line| line.parse::<u32>().expect("Could not parse line to int."))
		.collect::<Vec<u32>>();

	(
		Some(depths.array_windows().filter(|[a, b]| a < b).count().to_string()), 
		Some(depths.array_windows().filter(|[a, _, _, b]| a < b).count().to_string())
	)
}