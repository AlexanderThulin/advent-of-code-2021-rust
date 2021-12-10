pub fn main() {
    let binary_map = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.char_indices()
                .map(|(_, binary)| binary.to_digit(10).expect("Cound not parse binary"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    //PART ONE
    let mut sums = vec![0; binary_map[0].len()];

    for (_, binary_array) in binary_map.iter().enumerate() {
        for (i, binary) in binary_array.iter().enumerate() {
            sums[i] += binary;
        }
    }

    let gamma_string = sums
        .iter()
        .map(|sum| {
            if (*sum as usize) > binary_map.len() / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    let epsilon_string = gamma_string
        .chars()
        .map(|bit| if bit == '1' { '0' } else { '1' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_string, 2).unwrap();

    println!("Part one: {}", gamma * epsilon);

    // PART TWO
    let width: usize = binary_map[0].len();
    let mut oxy_temp = binary_map.clone();
    let mut co2_temp = binary_map.clone();

    for i in 0..width {
        let col_sum: u32 = oxy_temp.iter().map(|arr| arr.iter().nth(i).unwrap()).sum();
        let wanted = if (col_sum as f32) >= ((oxy_temp.len() as f32) / 2.0) {
            1
        } else {
            0
        };

        oxy_temp.retain(|row| row[i] == wanted);
        if oxy_temp.len() == 1 {
            break;
        }
    }

    for i in 0..width {
        let col_sum: u32 = co2_temp.iter().map(|arr| arr.iter().nth(i).unwrap()).sum();
        let wanted = if (col_sum as f32) >= ((co2_temp.len() as f32) / 2.0) {
            0
        } else {
            1
        };

        co2_temp.retain(|row| row[i] == wanted);
        if co2_temp.len() == 1 {
            break;
        }
    }

    let oxy = u32::from_str_radix(
        &oxy_temp[0]
            .iter()
            .map(|bit| bit.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let co2 = u32::from_str_radix(
        &co2_temp[0]
            .iter()
            .map(|bit| bit.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();

    println!("Part two: {}", oxy * co2);
}
