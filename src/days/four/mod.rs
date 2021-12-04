pub fn main() {
	let input = include_str!("input.txt").split("\r\n\r\n").collect::<Vec<&str>>();
	let draw = &input[0].split(",").collect::<Vec<_>>();
	let tickets = &input[1..]
		.iter()
		.map(|ticket| ticket
			.lines()
			.map(|line| line
				.split(" ")
				.filter(|number| number != &"")
				.collect::<Vec<_>>())
			.collect::<Vec<_>>())
		.collect::<Vec<_>>();

	let mut first_not_drawn = vec!();
	let mut last_not_drawn = vec!();
	let mut first_last_drawn = 0;
	let mut last_last_drawn = 0;
	let mut winning_tickets = vec!();

	for i in 0..draw.iter().len() {
		'ticket_check: for (ticket_number, ticket) in tickets.iter().enumerate() {
			if winning_tickets.iter().any(|tn| *tn == ticket_number) {
				continue 'ticket_check;
			}
			let mut found = 0;
			for row in ticket {
				'row_check: for number in row {
					if draw[0..i+1].iter().any(|n| n == number) {
						found += 1;
					} 
					else {
						break 'row_check;
					}
				}
				if found == 5 {
					let mut temp = vec!();
					for row in ticket {
						for number in row {
							if !draw[0..i+1].iter().any(|num| num == number) {
								temp.push(number.parse::<u32>().expect("Could not parse to number."));
							}
						}
					}
					if first_not_drawn.len() == 0 {
						first_not_drawn = temp;
						first_last_drawn = draw[i].parse::<u32>().expect("Could not parse number.");
					} 
					else {
						last_not_drawn = temp;
						last_last_drawn = draw[i].parse::<u32>().expect("Could not parse number.");
					}

					winning_tickets.push(ticket_number);
					continue 'ticket_check;
				}
				else {
					found = 0;
				}
			}
			for col_n in 0..5 {
				'col_check: for n in 0..5 {
					if draw[0..i+1].iter().any(|num| *num == ticket[n][col_n]) {
						found += 1;
					} 
					else {
						break 'col_check;
					}
				}
				if found == 5 {
					let mut temp = vec!();
					for row in ticket {
						for number in row {
							if !draw[0..i+1].iter().any(|num| num == number) {
								temp.push(number.parse::<u32>().expect("Could not parse to number."));
							}
						}
					}
					if first_not_drawn.len() == 0 {
						first_not_drawn = temp;
						first_last_drawn = draw[i].parse::<u32>().expect("Could not parse number.");
					} 
					else {
						last_not_drawn = temp;
						last_last_drawn = draw[i].parse::<u32>().expect("Could not parse number.");
					}

					winning_tickets.push(ticket_number);
					continue 'ticket_check;
				}
				else {
					found = 0;
				}
			}
		}
	}

	println!("Part one: {}", first_not_drawn.iter().sum::<u32>() * first_last_drawn);
	println!("Part two: {}", last_not_drawn.iter().sum::<u32>() * last_last_drawn);
}