use itertools::Itertools;

pub fn part1() -> usize {
    let input = include_str!("../../data/2021/day01.txt");
    let (_, count_increases) = input.lines().map(|n| n.parse::<usize>().unwrap()).fold(
        (None, 0),
        |(previous, count), current| match previous {
            Some(previous_value) => {
                if current > previous_value {
                    (Some(current), count + 1)
                } else {
                    (Some(current), count)
                }
            }
            None => (Some(current), count),
        },
    );
    count_increases
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2021/day01.txt");
    let input_numbers = input
        .lines()
        .into_iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    let grouped = groups_of_with_step(3, 1, input_numbers);
    let (_, count_increases) = grouped.iter().map(|v| v.iter().sum()).fold(
        (None, 0),
        |(previous, count): (Option<usize>, usize), current| match previous {
            Some(previous_value) => {
                if current > previous_value {
                    (Some(current), count + 1)
                } else {
                    (Some(current), count)
                }
            }
            None => (Some(current), count),
        },
    );

    count_increases
}

fn groups_of_with_step(size: usize, step: usize, it: Vec<usize>) -> Vec<Vec<usize>> {
    if size <= 0 || step <= 0 || it.len() < size {
        return vec![];
    }

    let this_group: Vec<usize> = it[0..size].iter().cloned().collect();

    let mut with_group = Vec::new();
    with_group.push(this_group);

    with_group.extend(groups_of_with_step(
        size,
        step,
        it[step..].iter().cloned().collect(),
    ));

    with_group
}
