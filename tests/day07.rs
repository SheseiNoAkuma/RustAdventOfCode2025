use advent_of_code::Day;
use advent_of_code::days::day07::Day07;

#[test]
fn part1_real_input() {
    let input = include_str!("../inputs/day07.txt");
    assert_eq!(Day07.part1(input).unwrap(), "1633");
}

#[test]
fn part2_real_input() {
    let input = include_str!("../inputs/day07.txt");
    assert_eq!(Day07.part2(input).unwrap(), "34339203133559");
}
