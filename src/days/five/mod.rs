pub fn main() {
    let lines_one = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|value| value.parse::<usize>().expect("Could not parse value."))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|line| line[0][0] == line[1][0] || line[0][1] == line[1][1])
        .collect::<Vec<_>>();

    let mut map_one = vec![vec![0u8; 1000]; 1000];
    for line in lines_one {
        let x1 = line[0][0];
        let y1 = line[0][1];
        let x2 = line[1][0];
        let y2 = line[1][1];

        if y1 == y2 {
            let low = if x1 < x2 { x1 } else { x2 };
            let high = if x1 > x2 { x1 } else { x2 };

            for x in low..=high {
                map_one[y1][x] += 1;
            }
        } else {
            let low = if y1 < y2 { y1 } else { y2 };
            let high = if y1 > y2 { y1 } else { y2 };

            (low..=high).for_each(|y| {
                map_one[y][x1] += 1;
            });
        }
    }

    let res_one = map_one
        .iter()
        .fold(0, |sum, x| sum + x.iter().filter(|p| **p > 1).count());

    let lines_two = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|value| value.parse::<usize>().expect("Could not parse value."))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut map_two = vec![vec![0u8; 1000]; 1000];
    for line in lines_two {
        let x1 = line[0][0];
        let y1 = line[0][1];
        let x2 = line[1][0];
        let y2 = line[1][1];

        if y1 == y2 {
            let low = if x1 < x2 { x1 } else { x2 };
            let high = if x1 > x2 { x1 } else { x2 };

            for x in low..=high {
                map_two[y1][x] += 1;
            }
        } else if x1 == x2 {
            let low = if y1 < y2 { y1 } else { y2 };
            let high = if y1 > y2 { y1 } else { y2 };

            (low..=high).for_each(|y| {
                map_two[y][x1] += 1;
            });
        } else if x1 < x2 {
            if y1 < y2 {
                for x in x1..=x2 {
                    map_two[y1 + (x - x1)][x] += 1;
                }
            } else {
                for x in x1..=x2 {
                    map_two[y1 - (x - x1)][x] += 1;
                }
            }
        } else if y1 < y2 {
            for x in x2..=x1 {
                map_two[y1 + (x1 - x)][x] += 1;
            }
        } else {
            for x in x2..=x1 {
                map_two[y1 - (x1 - x)][x] += 1;
            }
        }
    }

    let res_two = map_two
        .iter()
        .fold(0, |sum, x| sum + x.iter().filter(|p| **p > 1).count());

    println!("Result one: {}", res_one);
    println!("Result two: {}", res_two);
}
