use advent_of_code::Day;
use advent_of_code::days::day06::Day06;

#[test]
fn part1_real_input() {
    let input = include_str!("../inputs/day06.txt");
    assert_eq!(Day06.part1(input).unwrap(), "6209956042374");
}

#[test]
fn part2_real_input() {
    let input = include_str!("../inputs/day06.txt");
    assert_eq!(Day06.part2(input).unwrap(), "12608160008022");
}
