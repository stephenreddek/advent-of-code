#![feature(int_roundings)]

mod day23;

fn main() {
    let day23_part1 = day23::part1();
    let day23_part2 = day23::part2();
    std::println!(
        "Day23: part 1 \"{}\" part 2 \"{}\"",
        day23_part1,
        day23_part2
    );
}
