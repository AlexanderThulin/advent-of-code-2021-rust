pub fn main() {
    let depths = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<u32>().expect("Could not parse line to int."))
        .collect::<Vec<u32>>();

    let res_one = depths.array_windows().filter(|[a, b]| a < b).count();
    let res_two = depths.array_windows().filter(|[a, _, _, b]| a < b).count();

    println!("Part one: {}", res_one);
    println!("Part two: {}", res_two);
}
