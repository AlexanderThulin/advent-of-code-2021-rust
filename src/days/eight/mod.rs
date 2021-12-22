pub fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|part| part.split_whitespace().collect::<Vec<&str>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res_one = lines
        .iter()
        .flat_map(|line| &line[1])
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count();

    let res_two = lines
        .iter()
        .map(|line| translate_number(&line[0], &line[1]))
        .sum::<u32>();

    println!("Result one: {:?}", res_one);
    println!("Result two: {:?}", res_two);
}

fn translate_number(keys: &[&str], digits: &[&str]) -> u32 {
    let mut four: Vec<char> = vec![];
    let mut seven: Vec<char> = vec![];
    let mut res: Vec<&str> = vec![];

    for key in keys {
        match key.len() {
            4 => {
                four.extend(key.chars());
            }
            3 => {
                seven.extend(key.chars());
            }
            _ => {}
        }
    }

    for digit in digits {
        match (
            digit.len(),
            digit.chars().filter(|c| four.contains(c)).count(),
            digit.chars().filter(|c| seven.contains(c)).count(),
        ) {
            (2, _, _) => res.push("1"),
            (4, _, _) => res.push("4"),
            (3, _, _) => res.push("7"),
            (7, _, _) => res.push("8"),
            (5, _, 3) => res.push("3"),
            (5, 2, _) => res.push("2"),
            (5, _, _) => res.push("5"),
            (6, _, 2) => res.push("6"),
            (6, 4, _) => res.push("9"),
            (6, _, _) => res.push("0"),
            _ => {
                panic!("Can't translate digit.")
            }
        }
    }

    res.join("").parse::<u32>().unwrap()
}
