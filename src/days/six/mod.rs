pub fn main() {
    let fish_count = include_str!("input.txt")
        .split(",")
        .fold([0; 9], |mut total, fish_age| {
            total[fish_age
                .parse::<usize>()
                .expect("Could not parse fish age.")
                - 1] += 1;
            total
        });

    println!("Result one: {}", fish_tank(80, fish_count));
    println!("Result two: {}", fish_tank(256, fish_count));
}

fn fish_tank(days: u16, fishes: [usize; 9]) -> usize {
    let mut temp = fishes.clone();
    for _ in 0..days {
        temp.rotate_left(1);
        temp[5] += temp[7];
    }
    temp.iter().sum::<usize>()
}
