pub fn main() {
    let mut input = include_str!("input.txt")
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input.sort();
    let med = input[input.len() / 2];

    let res_one = input.iter().map(|p| (p - med).abs()).sum::<i32>();

    let res_two = (input.iter().sum::<i32>() / input.len() as i32..)
        .take(1)
        .map(|target| {
            input
                .iter()
                .map(|pos| {
                    let diff = (target - pos).abs();
                    diff * (diff + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("Result one: {}", res_one);
    println!("Result one: {}", res_two);
}
