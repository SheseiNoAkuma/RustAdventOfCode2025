use advent_of_code::Day;
use advent_of_code::days::day02::Day02;

#[test]
fn day02_part1_real_input() {
    let input = include_str!("../inputs/day02.txt");
    assert_eq!(Day02.part1(input).unwrap(), "37314786486");
}

