#![feature(int_roundings)]

mod day21;

fn main() {
    let day21_part1 = day21::part1();
    let day21_part2 = day21::part2();
    std::println!(
        "Day21: part 1 \"{}\" part 2 \"{}\"",
        day21_part1,
        day21_part2
    );
}
