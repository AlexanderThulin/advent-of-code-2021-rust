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

fn translate_number(keys: &Vec<&str>, digits: &Vec<&str>) -> u32 {
    let mut four: Vec<char> = vec![];
    let mut seven: Vec<char> = vec![];
    let mut res: Vec<&str> = vec![];

    for key in keys {
        match key.len() {
            4 => {
                for character in key.chars() {
                    four.push(character);
                }
            }
            3 => {
                for character in key.chars() {
                    seven.push(character);
                }
            }
            _ => {}
        }
    }

    for digit in digits {
        match digit.len() {
            2 => res.push("1"),
            4 => res.push("4"),
            3 => res.push("7"),
            7 => res.push("8"),
            5 if digit.chars().filter(|c| seven.contains(c)).count() == 3 => res.push("3"),
            5 if digit.chars().filter(|c| four.contains(c)).count() == 2 => res.push("2"),
            5 => res.push("5"),
            6 if digit.chars().filter(|c| seven.contains(c)).count() == 2 => res.push("6"),
            6 if digit.chars().filter(|c| four.contains(c)).count() == 4 => res.push("9"),
            6 => res.push("0"),
            _ => {
                panic!("Unexpected length encountered.")
            }
        }
    }

    res.join("").parse::<u32>().unwrap()
}
