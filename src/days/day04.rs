use crate::{AocResult, Day};

pub struct Day04;
impl Day for Day04 {
    fn name(&self) -> &'static str {
        "day04"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let lines: Vec<Vec<bool>> = convert_input(input);

        let count: i32 = lines
            .iter()
            .enumerate()
            .flat_map(|(row, row_vec)| {
                row_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(col, &v)| {
                        if v && count_adjacent(row as u16, col as u16, &lines) < 4 {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .sum();

        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        let mut lines: Vec<Vec<bool>> = convert_input(input);
        let mut should_continue = true;
        let mut count: i32 = 0;

        while should_continue {
            let count_before = count_total_true(&lines);

            lines = lines
                .iter()
                .enumerate()
                .map(|(row, row_vec)| {
                    row_vec
                        .iter()
                        .enumerate()
                        .map(|(col, &v)| match v {
                            false => false,
                            true if count_adjacent(row as u16, col as u16, &lines) < 4 => false,
                            true => true,
                        })
                        .collect()
                })
                .collect();

            let count_after = count_total_true(&lines);

            count += count_before - count_after;
            should_continue = count_after != count_before;
        }

        Ok(count.to_string())
    }
}

fn count_total_true(grid: &Vec<Vec<bool>>) -> i32 {
    grid.iter()
        .map(|r| r.iter().filter(|&&v| v).count() as i32)
        .sum::<i32>()
}

fn convert_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => true,
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_adjacent(row: u16, col: u16, grid: &Vec<Vec<bool>>) -> u16 {
    DELTAS
        .iter()
        .filter_map(|(deviance_row, deviance_column)| {
            let adjacent_row = row as isize + deviance_row;
            let adjacent_column = col as isize + deviance_column;

            if adjacent_row < 0 || adjacent_column < 0 {
                return None;
            }

            grid.get(adjacent_row as usize)
                .and_then(|row: &Vec<bool>| row.get(adjacent_column as usize))
                .copied()
        })
        .filter(|&v| v)
        .count() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn count_adjacent_test() {
        let grid = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];

        assert_eq!(count_adjacent(1, 1, &grid), 4);
        assert_eq!(count_adjacent(0, 1, &grid), 3);
    }

    #[test]
    /**
    The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions.
    ..xx.xx@x.
    x@@.@.@.@@
    @@@@@.x.@@
    @.@@@@..@.
    x@.@@@@.@x
    .@@@@@@@.@
    .@.@.@.@@@
    x.@@@.@@@@
    .@@@@@@@@.
    x.x.@@@.x.
     */
    fn part1_test() {
        assert_eq!(Day04.part1(INPUT).unwrap(), "13");
    }

    #[test]
    /**
    Initial state:
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.

    Remove 13 rolls of paper:
    ..xx.xx@x.
    x@@.@.@.@@
    @@@@@.x.@@
    @.@@@@..@.
    x@.@@@@.@x
    .@@@@@@@.@
    .@.@.@.@@@
    x.@@@.@@@@
    .@@@@@@@@.
    x.x.@@@.x.

    Remove 12 rolls of paper:
    .......x..
    .@@.x.x.@x
    x@@@@...@@
    x.@@@@..x.
    .@.@@@@.x.
    .x@@@@@@.x
    .x.@.@.@@@
    ..@@@.@@@@
    .x@@@@@@@.
    ....@@@...

    Remove 7 rolls of paper:
    ..........
    .x@.....x.
    .@@@@...xx
    ..@@@@....
    .x.@@@@...
    ..@@@@@@..
    ...@.@.@@x
    ..@@@.@@@@
    ..x@@@@@@.
    ....@@@...

    Remove 5 rolls of paper:
    ..........
    ..x.......
    .x@@@.....
    ..@@@@....
    ...@@@@...
    ..x@@@@@..
    ...@.@.@@.
    ..x@@.@@@x
    ...@@@@@@.
    ....@@@...

    Remove 2 rolls of paper:
    ..........
    ..........
    ..x@@.....
    ..@@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@x.
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ...@@.....
    ..x@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ...x@.....
    ...@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ....x.....
    ...@@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    Remove 1 roll of paper:
    ..........
    ..........
    ..........
    ...x@@....
    ...@@@@...
    ...@@@@@..
    ...@.@.@@.
    ...@@.@@@.
    ...@@@@@..
    ....@@@...

    total of 43 rolls of paper can be removed
     */
    fn part2_test() {
        assert_eq!(Day04.part2(INPUT).unwrap(), "43");
    }
}
