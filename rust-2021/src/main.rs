mod day01;

fn main() {
    let day1_part1_result = day01::part1();
    let day1_part2_result = day01::part2();
    std::println!(
        "Day 1: part 1 \"{}\" part 2 \"{}\"",
        day1_part1_result,
        day1_part2_result
    );
}
