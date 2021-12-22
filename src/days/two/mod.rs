pub fn main() {
    let directions: Vec<(&str, u32)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let line_split: Vec<&str> = line.split(' ').collect();
            (
                line_split[0],
                line_split[1]
                    .parse::<u32>()
                    .expect("Could not parse directions."),
            )
        })
        .collect();

    // PART ONE
    let (horizontal, depth) = directions.iter().fold(
        (0, 0),
        |(horizontal, depth), (direction, value)| match *direction {
            "forward" => (horizontal + value, depth),
            "down" => (horizontal, depth + value),
            "up" => (horizontal, depth - value),
            _ => unreachable!(),
        },
    );
    let res_one = horizontal * depth;

    // PART TWO
    let (horizontal, depth, _) =
        directions
            .iter()
            .fold(
                (0, 0, 0),
                |(horizontal, depth, aim), (direction, value)| match *direction {
                    "forward" => (horizontal + value, depth + aim * value, aim),
                    "down" => (horizontal, depth, aim + value),
                    "up" => (horizontal, depth, aim - value),
                    _ => unreachable!(),
                },
            );
    let res_two = horizontal * depth;

    println!("Part one: {}", res_one);
    println!("Part two: {}", res_two);
}
