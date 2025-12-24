use crate::{AocResult, Day};
use std::iter::Map;
use std::slice::Iter;
use std::str::Lines;

pub struct Day03;
impl Day for Day03 {
    fn name(&self) -> &'static str {
        "day03"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let batteries = Self::convert_batteries(input);

        Ok(batteries
            .map(|b| find_max_voltage(b).unwrap())
            .sum::<u32>()
            .to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        let batteries = Self::convert_batteries(input);
        Ok(batteries
            .map(|b| find_max_voltage_overclock(b).unwrap())
            .sum::<u64>()
            .to_string())
    }
}

impl Day03 {
    fn convert_batteries(input: &'_ str) -> Map<Lines<'_>, fn(&str) -> Vec<u32>> {
        input.lines().map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
    }
}

fn find_max_voltage_overclock(battery: Vec<u32>) -> Option<u64> {
    let (_, digits) = (0..12)
        .rev()
        .fold((0usize, String::new()), |(last_index, mut s), i| {
            let slice: Iter<u32> = battery[last_index..battery.len().saturating_sub(i)].iter();

            let (current_digit_index, current_digit_val) = find_max_in_range(slice).unwrap();

            s.push_str(&current_digit_val.to_string());
            (last_index + current_digit_index + 1, s)
        });

    Some(digits.parse::<u64>().unwrap())
}

fn find_max_in_range(slice: Iter<u32>) -> Option<(usize, u32)> {
    let (first_digit_index, first_digit_val) = slice
        .enumerate()
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then_with(|| i2.cmp(i1)))?;

    Some((first_digit_index, *first_digit_val))
}

fn find_max_voltage(battery: Vec<u32>) -> Option<u32> {
    let slice: Iter<u32> = battery[0..battery.len().saturating_sub(1)].iter();
    let (first_digit_index, first_digit_val) = find_max_in_range(slice)?;

    let second_digit = battery.iter().skip(first_digit_index + 1).copied().max()?;
    let voltage = (first_digit_val.to_string() + &second_digit.to_string())
        .parse()
        .unwrap();

    Some(voltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn find_max_voltage_overclock_test() {
        assert_eq!(
            find_max_voltage_overclock(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            Some(987654321111)
        );
        assert_eq!(
            find_max_voltage_overclock(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            Some(811111111119)
        );
        assert_eq!(
            find_max_voltage_overclock(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            Some(434234234278)
        );
        assert_eq!(
            find_max_voltage_overclock(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            Some(888911112111)
        );
    }

    #[test]
    fn find_max_voltage_test() {
        assert_eq!(
            find_max_voltage(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            Some(98)
        );
        assert_eq!(
            find_max_voltage(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            Some(89)
        );
        assert_eq!(
            find_max_voltage(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            Some(78)
        );
        assert_eq!(
            find_max_voltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            Some(92)
        );
    }

    #[test]
    fn find_max_voltage_test_2() {
        assert_eq!(
            find_max_voltage(vec![
                3, 4, 3, 4, 8, 4, 5, 6, 3, 4, 4, 5, 4, 3, 6, 4, 5, 4, 6, 3, 3, 4, 3, 3, 5, 3, 3, 3,
                4, 4, 8, 4, 4, 3, 3, 5, 4, 3, 2, 4, 5, 3, 3, 5, 4, 5, 4, 4, 3, 2, 3, 5, 4, 1, 4, 3,
                3, 4, 4, 7, 7, 4, 2, 4, 4, 4, 2, 4, 4, 4, 3, 4, 6, 8, 4, 4, 3, 4, 4, 2, 4, 4, 4, 4,
                4, 4, 3, 4, 4, 4, 5, 3, 3, 3, 3, 4, 4, 3, 1, 4
            ]),
            Some(88)
        );
    }

    #[test]
    /**
    In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
    In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
    In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
    In 818181911112111, the largest joltage you can produce is 92.
    output joltage is 98 + 89 + 78 + 92 = 357.
     */
    fn day03_part1_test() {
        assert_eq!(Day03.part1(INPUT).unwrap(), "357");
    }
    #[test]
    /**
    In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
    In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
    In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
    In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
    joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
     */
    fn day03_part2_test() {
        assert_eq!(Day03.part2(INPUT).unwrap(), "3121910778619");
    }
}
