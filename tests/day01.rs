use advent_of_code::Day;
use advent_of_code::days::day01::Day01;

#[test]
fn part1_real_input() {
    let input = include_str!("../inputs/day01.txt");
    assert_eq!(Day01.part1(input).unwrap(), "1132");
}

#[test]
fn part2_real_input() {
    let input = include_str!("../inputs/day01.txt");
    assert_eq!(Day01.part2(input).unwrap(), "6623");
}
