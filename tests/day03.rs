use advent_of_code::Day;
use advent_of_code::days::day03::Day03;

#[test]
fn day03_part1_real_input() {
    let input = include_str!("../inputs/day03.txt");
    assert_eq!(Day03.part1(input).unwrap(), "17107");
}

#[test]
fn day03_part2_real_input() {
    let input = include_str!("../inputs/day03.txt");
    assert_eq!(Day03.part2(input).unwrap(), "169349762274117");
}
