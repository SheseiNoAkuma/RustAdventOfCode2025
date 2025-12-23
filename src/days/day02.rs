use crate::{AocResult, Day};

pub struct Day02;
impl Day02 {

    fn solve_with<F>(&self, input: &str, is_invalid: F) -> AocResult<String>
    where
        F: Fn(u64) -> bool,
    {
        let sequences: Vec<Vec<u64>> = input
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut it = s.split('-').map(|n| n.parse::<u64>().unwrap());
                let start = it.next().unwrap();
                let end = it.next().unwrap();
                (start..=end).collect()
            })
            .collect();

        let sum: u64 = sequences
            .iter()
            .flat_map(|seq| seq.iter())
            .copied()
            .filter(|n| is_invalid(*n))
            .sum();

        Ok(sum.to_string())
    }


    fn is_invalid_number_p1(n: u64) -> bool {
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

    fn is_invalid_number_p2(n: u64) -> bool {
        Day02::is_invalid_number_p1(n) || Day02::is_number_duplicated_n_times(n)
    }

    fn is_number_duplicated_n_times(n: u64) -> bool {
        let as_str = n.to_string();
        let to: usize = as_str.len() / 2;
        for i in 1..=to {
            let (a, b) = as_str.split_at(i);
            let c = Day02::split_by_len(b, a.len());
            if c.iter().filter(|s| s.as_str() != a).count() == 0 {
                println!("{} IS INVALID cause {} is in {}", as_str, a, b);
                return true;
            }
        }
        false
    }

    fn split_by_len(s: &str, len: usize) -> Vec<String> {
        assert!(len > 0);

        s.chars()
            .collect::<Vec<_>>()
            .chunks(len)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }
}
impl Day for Day02 {
    fn name(&self) -> &'static str {
        "day02"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        self.solve_with(input, Day02::is_invalid_number_p1)
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        self.solve_with(input, Day02::is_invalid_number_p2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn is_invalid_number_true() {
        assert_eq!(Day02::is_invalid_number_p1(11), true);
        assert_eq!(Day02::is_invalid_number_p1(22), true);
        assert_eq!(Day02::is_invalid_number_p1(55), true);
        assert_eq!(Day02::is_invalid_number_p1(6464), true);
        assert_eq!(Day02::is_invalid_number_p1(123123), true);
    }

    #[test]
    fn is_invalid_number_false() {
        assert_eq!(Day02::is_invalid_number_p1(7), false);
        assert_eq!(Day02::is_invalid_number_p1(21), false);
        assert_eq!(Day02::is_invalid_number_p1(156), false);
        assert_eq!(Day02::is_invalid_number_p1(784), false);
        assert_eq!(Day02::is_invalid_number_p1(123143), false);
        //
        assert_eq!(Day02::is_invalid_number_p1(999), false);
        assert_eq!(Day02::is_invalid_number_p1(1111111), false);
    }

    #[test]
    fn is_invalid_number_p2_true() {
        assert_eq!(Day02::is_number_duplicated_n_times(999), true);
        assert_eq!(Day02::is_number_duplicated_n_times(1111111), true);
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
    fn part1_test() {
        assert_eq!(Day02.part1(INPUT).unwrap(), "1227775554");
    }

    /**
    11-22 still has two invalid IDs, 11 and 22.
    95-115 now has two invalid IDs, 99 and 111.
    998-1012 now has two invalid IDs, 999 and 1010.
    1188511880-1188511890 still has one invalid ID, 1188511885.
    222220-222224 still has one invalid ID, 222222.
    1698522-1698528 still contains no invalid IDs.
    446443-446449 still has one invalid ID, 446446.
    38593856-38593862 still has one invalid ID, 38593859.
    565653-565659 now has one invalid ID, 565656.
    824824821-824824827 now has one invalid ID, 824824824.
    2121212118-2121212124 now has one invalid ID, 2121212121.
    */
    #[test]
    fn part2_test() {
        assert_eq!(Day02.part2(INPUT).unwrap(), "4174379265");
    }
}
