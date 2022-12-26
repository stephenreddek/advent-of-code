#![feature(int_roundings)]

mod day25;

fn main() {
    let day25_part1 = day25::part1();
    let day25_part2 = day25::part2();
    std::println!(
        "Day25: part 1 \"{}\" part 2 \"{}\"",
        day25_part1,
        day25_part2
    );
}
