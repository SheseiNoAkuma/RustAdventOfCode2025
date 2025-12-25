use advent_of_code::Day;
use advent_of_code::days::day05::Day05;

#[test]
fn part1_real_input() {
    let input = include_str!("../inputs/day05.txt");
    assert_eq!(Day05.part1(input).unwrap(), "896");
}

#[test]
fn part2_real_input() {
    let input = include_str!("../inputs/day05.txt");
    assert_eq!(Day05.part2(input).unwrap(), "346240317247002");
}
