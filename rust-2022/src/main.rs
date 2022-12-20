#![feature(int_roundings)]

mod day19;

fn main() {
    let day19_part1 = day19::part1();
    let day19_part2 = day19::part2();
    std::println!(
        "Day19: part 1 \"{}\" part 2 \"{}\"",
        day19_part1,
        day19_part2
    );
}
