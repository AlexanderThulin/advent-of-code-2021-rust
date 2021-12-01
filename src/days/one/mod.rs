pub fn run(lines: Vec<String>) -> (Option<String>, Option<String>) {
	let depths: Vec<u32> = lines.iter().map(|line| line.parse::<u32>().unwrap()).collect();
	
	let mut res_one: u16 = 0;
	depths.iter().enumerate().for_each(|(i, depth)| {
		if i+1 < depths.len() && depth < &depths[i+1] { res_one += 1; }
	});

 let mut res_two: u16 = 0;
 depths.iter().enumerate().for_each(|(i, _)| {
	 if i+3 < depths.len() {
		 let a: u32 = depths[i..i+3].iter().sum();
		 let b: u32 = depths[i+1..i+4].iter().sum();

		 if a < b {
			 res_two += 1;
		 }
	 }
 });

 (Some(res_one.to_string()), Some(res_two.to_string()))
}