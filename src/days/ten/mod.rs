pub fn main() {
    let res_one = include_str!("input.txt")
        .lines()
        .map(|line| line.chars())
        .map(|line| read_nav_line(line.collect(), true))
        .sum::<u64>();

    let mut incomplete = include_str!("input.txt")
        .lines()
        .map(|line| line.chars())
        .map(|line| read_nav_line(line.collect(), false))
        .filter(|n| *n != 0)
        .collect::<Vec<_>>();
    incomplete.sort_unstable();
    let res_two = incomplete[incomplete.len() / 2];

    println!("Part one: {}", res_one);
    println!("Part two: {:?}", res_two);
}

fn read_nav_line(line: Vec<char>, corrupted: bool) -> u64 {
    let mut expected = vec![];
    for c in line {
        match c {
            '(' => expected.push(')'),
            '[' => expected.push(']'),
            '{' => expected.push('}'),
            '<' => expected.push('>'),
            _ => {
                if !expected.is_empty() && c == *expected.last().unwrap() {
                    expected.pop();
                } else if expected.is_empty() {
                    continue;
                } else {
                    return if corrupted {
                        match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Bad match"),
                        }
                    } else {
                        0
                    };
                }
            }
        }
    }
    expected.reverse();
    return if corrupted {
        0
    } else {
        expected.iter().fold(0, |sum, e| {
            (sum * 5)
                + match e {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Bad match"),
                }
        })
    };
}
