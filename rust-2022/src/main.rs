#![feature(int_roundings)]

mod day20;

fn main() {
    let day20_part1 = day20::part1();
    let day20_part2 = day20::part2();
    std::println!(
        "Day20: part 1 \"{}\" part 2 \"{}\"",
        day20_part1,
        day20_part2
    );
}
