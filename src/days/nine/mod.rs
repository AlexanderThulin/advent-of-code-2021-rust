pub fn main() {
	let map = include_str!("input.txt")
		.lines()
		.map(|line| line
			.chars()
			.map(|n| n
				.to_digit(10)
				.unwrap())
			.collect::<Vec<_>>())
		.collect::<Vec<_>>();

	// PART ONE
	let mut lows = vec!();
	for (i, row) in map.iter().enumerate() {
		for (j, n) in row.iter().enumerate() {
			if i > 0 && map[i - 1][j] <= *n { continue; }
			if i < map.len() - 1 && map[i + 1][j] <= *n { continue; }
			if j > 0 && map[i][j - 1] <= *n { continue; }
			if j < map.len() - 1 && map[i][j + 1] <= *n { continue; }

			lows.push(*n + 1);
		}
	}

	println!("Result one: {}", lows.iter().sum::<u32>());

	// PART TWO
	let mut basins = map
		.iter()
		.map(|row| row
			.iter()
			.map(|n| if *n == 9 { u16::MAX } else { 0 })
			.collect::<Vec<_>>())
		.collect::<Vec<_>>();
 

	let height = basins.len();
	let width = basins[0].len();
	let mut n =  1;

	for y in 0..height {
		for x in 0..width {
			fill_basin(x, y, n, &mut basins);
			n += 1;
		}
	}
	
	let mut basin_n = basins
			.iter()
			.flat_map(|n| n)
			.filter(|n| *n != &u16::MAX)
			.collect::<Vec<_>>();
	basin_n.sort_unstable_by(|a, b| b.cmp(a));

	let mut basin_sizes = basin_n.group_by(|a, b| a == b).map(|v| v.len()).collect::<Vec<_>>();
	basin_sizes.sort_unstable_by(|a, b| b.cmp(a));

	let res_two = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

	println!("Result two: {}", res_two);
}

fn fill_basin(x: usize, y: usize, n: u16, map: &mut Vec<Vec<u16>>) {
		if map[y][x] == 0 {
			map[y][x] = n;

			if y > 0 { fill_basin(x, y - 1, n, map) }
			if y < map.len() - 1 { fill_basin(x, y + 1, n, map) }
			if x > 0 { fill_basin(x - 1, y, n, map) }
			if x < map.len() - 1 { fill_basin(x + 1, y, n, map) }
		}
	}