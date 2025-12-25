use crate::{AocResult, Day};
use std::ops::RangeInclusive;

pub struct Day05;
impl Day for Day05 {
    fn name(&self) -> &'static str {
        "day05"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let (ranges, ingredients) = parse_input(input.lines().collect());

        let fresh_ingredients = ingredients
            .iter()
            .filter(|i| ranges.iter().any(|r| r.contains(i)));

        Ok(fresh_ingredients.count().to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        let (ranges, _) = parse_input(input.lines().collect());

        let converted_ranges: Vec<_> = ranges
            .into_iter()
            .map(|r| Interval {
                start: *r.start(),
                end: *r.end(),
            })
            .collect();

        let count = count_distinct(converted_ranges);

        Ok(count.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    start: i64,
    end: i64, // inclusivo
}

impl Interval {
    fn normalized(self) -> Self {
        if self.start <= self.end {
            self
        } else {
            Self { start: self.end, end: self.start }
        }
    }

    fn len_u128(self) -> u128 {
        // len inclusiva: end - start + 1 (start <= end)
        (self.end as i128 - self.start as i128 + 1) as u128
    }
}

fn count_distinct(mut intervals: Vec<Interval>) -> u128 {
    if intervals.is_empty() {
        return 0;
    }

    // Normalizza e ordina
    for it in &mut intervals {
        *it = it.normalized();
    }
    intervals.sort_unstable_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));

    let mut total: u128 = 0;

    let mut cur = intervals[0];
    for next in intervals.into_iter().skip(1) {
        // overlap o contiguo?
        if next.start <= cur.end.saturating_add(1) {
            if next.end > cur.end {
                cur.end = next.end;
            }
        } else {
            total += cur.len_u128();
            cur = next;
        }
    }
    total += cur.len_u128();
    total
}


fn parse_input(input: Vec<&str>) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut split = input
        .splitn(2, |s| s.is_empty())
        .map(|s| s.iter().collect::<Vec<_>>());

    let ranges: Vec<RangeInclusive<i64>> = split
        .next()
        .unwrap()
        .into_iter()
        .map(|i| str_to_inclusive_range(i))
        .collect();

    let ingredients: Vec<_> = split
        .next()
        .unwrap()
        .into_iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect();

    (ranges, ingredients)
}

fn str_to_inclusive_range(s: &str) -> RangeInclusive<i64> {
    let mut it = s.split('-').map(|n| n.parse::<i64>().unwrap());
    let start = it.next().unwrap();
    let end = it.next().unwrap();
    start..=end
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn parse_input_test() {
        let expected = (
            vec![3..=5, 10..=14, 16..=20, 12..=18],
            vec![1, 5, 8, 11, 17, 32],
        );
        assert_eq!(parse_input(INPUT.lines().collect()), expected);
    }

    #[test]
    fn part1_test() {
        assert_eq!(Day05.part1(INPUT).unwrap(), "3");
    }

    #[test]
    /**
    Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

    3-5
    10-14
    16-20
    12-18
    The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20.
    So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.
    */
    fn part2_test() {
        assert_eq!(Day05.part2(INPUT).unwrap(), "14");
    }
}
