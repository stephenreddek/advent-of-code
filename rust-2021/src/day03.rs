use std::collections::HashSet;

#[derive(Clone)]
struct Counts {
    zeros: i32,
    ones: i32,
}

pub fn part1() -> i32 {
    let input = include_str!("../../data/2021/day03.txt");
    let length = input.lines().next().unwrap().len();

    let mut frequency: Vec<Counts> = vec![Counts { zeros: 0, ones: 0 }; length];

    input.lines().for_each(|line| {
        for (index, char) in line.char_indices() {
            if char == '1' {
                frequency[index].ones += 1;
            } else {
                frequency[index].zeros += 1;
            }
        }
    });

    let gamma_rate = frequency.iter().fold(0, |val_so_far, counts: &Counts| {
        (val_so_far << 1) + (if counts.ones > counts.zeros { 1 } else { 0 })
    });

    let epsilon_rate = frequency.iter().fold(0, |val_so_far, counts: &Counts| {
        (val_so_far << 1) + (if counts.ones < counts.zeros { 1 } else { 0 })
    });

    gamma_rate * epsilon_rate
}

pub fn part2() -> i32 {
    let input = include_str!("../../data/2021/day03.txt");
    let length = input.lines().next().unwrap().len();

    let mut remaining_lines_for_oxygen: HashSet<String> = input
        .lines()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    let mut remaining_lines_for_co2: HashSet<String> = remaining_lines_for_oxygen.clone();

    let mut current_bit: usize = 0;
    while remaining_lines_for_oxygen.len() > 1 && current_bit < length {
        let counts = frequency_at_bit(&remaining_lines_for_oxygen, current_bit);
        remaining_lines_for_oxygen.retain(|val| {
            if counts.ones >= counts.zeros {
                val.chars().nth(current_bit) == Some('1')
            } else {
                val.chars().nth(current_bit) == Some('0')
            }
        });

        current_bit += 1;
    }

    assert!(remaining_lines_for_oxygen.len() == 1);

    current_bit = 0;
    while remaining_lines_for_co2.len() > 1 && current_bit < length {
        let counts = frequency_at_bit(&remaining_lines_for_co2, current_bit);
        remaining_lines_for_co2.retain(|val| {
            if counts.ones >= counts.zeros {
                val.chars().nth(current_bit) == Some('0')
            } else {
                val.chars().nth(current_bit) == Some('1')
            }
        });

        current_bit += 1;
    }

    assert!(remaining_lines_for_co2.len() == 1);

    let oxygen_line = remaining_lines_for_oxygen.iter().next().unwrap();
    let co2_line = remaining_lines_for_co2.iter().next().unwrap();

    let oxygen_rate = oxygen_line.chars().fold(0, |val_so_far, bit| {
        (val_so_far << 1) + (if bit == '1' { 1 } else { 0 })
    });

    let co2_scrub_rate = co2_line.chars().fold(0, |val_so_far, bit| {
        (val_so_far << 1) + (if bit == '1' { 1 } else { 0 })
    });

    oxygen_rate * co2_scrub_rate
}

fn frequency_at_bit(remaining: &HashSet<String>, bit: usize) -> Counts {
    remaining
        .iter()
        .fold(Counts { ones: 0, zeros: 0 }, |counts, line| {
            match line.chars().nth(bit) {
                Some('1') => Counts {
                    ones: counts.ones + 1,
                    zeros: counts.zeros,
                },
                Some('0') => Counts {
                    ones: counts.ones,
                    zeros: counts.zeros + 1,
                },
                _ => counts,
            }
        })
}
