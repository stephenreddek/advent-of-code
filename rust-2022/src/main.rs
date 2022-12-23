#![feature(int_roundings)]

mod day22;

fn main() {
    let day22_part1 = day22::part1();
    let day22_part2 = day22::part2();
    std::println!(
        "Day22: part 1 \"{}\" part 2 \"{}\"",
        day22_part1,
        day22_part2
    );
}
