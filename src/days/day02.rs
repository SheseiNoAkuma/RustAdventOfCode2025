use crate::{AocResult, Day};

pub struct Day02;
impl Day02 {
    fn is_invalid_number(n: u64) -> bool {
        let as_str = n.to_string();
        let to: usize = as_str.len() / 2;
        for i in 1..=to {
            let (a, b) = as_str.split_at(i);
            if a == b {
                println!("{} IS INVALID cause {} == {}", as_str, a, b);
                return true;
            }
        }
        false
    }
}
impl Day for Day02 {
    fn name(&self) -> &'static str {
        "day02"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let sequences: Vec<_> = input
            .split(",")
            .map(|s| s.trim())
            .filter(|s| s.is_empty() == false)
            .map(|s| {
                s.split("-")
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|seq| (seq[0]..=seq[1]).collect::<Vec<_>>())
            .collect();

        let invalid_numbers: Vec<&u64> = sequences
            .iter()
            .flat_map(|seq| seq.iter().filter(|n| Day02::is_invalid_number(**n)))
            .collect::<Vec<_>>();
        let sum: u64 = invalid_numbers.into_iter().sum();
        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn is_invalid_number_true() {
        assert_eq!(Day02::is_invalid_number(11), true);
        assert_eq!(Day02::is_invalid_number(22), true);
        assert_eq!(Day02::is_invalid_number(55), true);
        assert_eq!(Day02::is_invalid_number(6464), true);
        assert_eq!(Day02::is_invalid_number(123123), true);
    }

    #[test]
    fn is_invalid_number_false() {
        assert_eq!(Day02::is_invalid_number(7), false);
        assert_eq!(Day02::is_invalid_number(21), false);
        assert_eq!(Day02::is_invalid_number(156), false);
        assert_eq!(Day02::is_invalid_number(784), false);
        assert_eq!(Day02::is_invalid_number(123143), false);
    }

    #[test]
    /**
    11-22 has two invalid IDs, 11 and 22.
    95-115 has one invalid ID, 99.
    998-1012 has one invalid ID, 1010.
    1188511880-1188511890 has one invalid ID, 1188511885.
    222220-222224 has one invalid ID, 222222.
    1698522-1698528 contains no invalid IDs.
    446443-446449 has one invalid ID, 446446.
    38593856-38593862 has one invalid ID, 38593859.
    The rest of the ranges contain no invalid IDs.
    */
    fn part1_counts_lines() {
        assert_eq!(Day02.part1(INPUT).unwrap(), "1227775554");
    }
}
