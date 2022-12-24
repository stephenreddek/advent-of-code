#![feature(int_roundings)]

mod day24;

fn main() {
    let day24_part1 = day24::part1();
    let day24_part2 = day24::part2();
    std::println!(
        "Day24: part 1 \"{}\" part 2 \"{}\"",
        day24_part1,
        day24_part2
    );
}
