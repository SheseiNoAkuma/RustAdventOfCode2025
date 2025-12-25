use advent_of_code::Day;
use advent_of_code::days::day04::Day04;

#[test]
fn part1_real_input() {
    let input = include_str!("../inputs/day04.txt");
    assert_eq!(Day04.part1(input).unwrap(), "1344");
}

#[test]
fn part2_real_input() {
    let input = include_str!("../inputs/day04.txt");
    assert_eq!(Day04.part2(input).unwrap(), "8112");
}
