use crate::{AocResult, Day};
use std::fmt;

pub struct Day07;
impl Day for Day07 {
    fn name(&self) -> &'static str {
        "day07"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let mut grid = parse_input(input);
        let (start_x, start_y) = grid.find_start();

        let first = grid.add_tachyon(start_x, start_y + 1);
        let other = ((start_y + 2)..grid.rows())
            .map(|y| grid.propagate_tachyon_to_row(y))
            .sum::<i32>();
        println!("{}", grid);
        Ok(format!("{}", first + other))
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        let table = input.trim().as_bytes();
        let columns = table
            .iter()
            .position(|&tile| tile == b'\n')
            .expect("Invalid input");

        let mut state = [1u64; 200];
        for row in table.chunks(columns + 1).rev() {
            let mut new_state = [0; 200];
            for (c, &tile) in row.iter().take(columns).enumerate() {
                if tile == b'^' {
                    new_state[c] = c.checked_sub(1).map_or(0, |c| state[c]) + {
                        let c = c + 1;
                        if c < columns { state[c] } else { 0 }
                    };
                } else if tile == b'S' {
                    return Ok(state[c].to_string());
                } else {
                    new_state[c] = state[c];
                }
            }

            state = new_state;
        }

        unreachable!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Quadrant {
    Start,
    Empty,
    Splitter,
    Tachyon,
}
impl Quadrant {
    fn matches(&self, q: Quadrant) -> bool {
        match (self, q) {
            (Quadrant::Empty, Quadrant::Empty)
            | (Quadrant::Start, Quadrant::Start)
            | (Quadrant::Splitter, Quadrant::Splitter)
            | (Quadrant::Tachyon, Quadrant::Tachyon) => true,
            (_, _) => false,
        }
    }
}
impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quadrant::Start => write!(f, "A"),
            Quadrant::Empty => write!(f, "E"),
            Quadrant::Splitter => write!(f, "S"),
            Quadrant::Tachyon => write!(f, "T"),
        }
    }
}
impl From<char> for Quadrant {
    fn from(c: char) -> Self {
        match c {
            '.' => Quadrant::Empty,
            '^' => Quadrant::Splitter,
            'S' => Quadrant::Start,
            _ => panic!("invalid quadrant: {}", c),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct InputTree {
    quadrants: Vec<Vec<Quadrant>>,
}
impl InputTree {
    fn rows(&self) -> usize {
        self.quadrants.len()
    }

    fn find_start(&self) -> (usize, usize) {
        self.quadrants
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, q)| (q == &Quadrant::Start).then(|| (x, y)))
            })
            .unwrap()
    }

    fn propagate_tachyon_to_row(&mut self, y: usize) -> i32 {
        self.clone()
            .find_tachyons_in_row(y - 1)
            .iter()
            .map(|x| self.add_tachyon(*x, y))
            .sum()
    }

    fn find_tachyons_in_row(self, y: usize) -> Vec<usize> {
        self.quadrants[y]
            .iter()
            .enumerate()
            .filter_map(|(x, q)| q.matches(Quadrant::Tachyon).then(|| x))
            .collect()
    }

    fn add_tachyon(&mut self, x: usize, y: usize) -> i32 {
        match self.quadrants[y][x] {
            Quadrant::Empty => {
                self.quadrants[y][x] = Quadrant::Tachyon;
                0
            }
            Quadrant::Splitter => {
                self.quadrants[y][x - 1] = Quadrant::Tachyon;
                self.quadrants[y][x + 1] = Quadrant::Tachyon;
                1
            }
            Quadrant::Tachyon => 0,
            q => panic!("quadrant in unexpected state at ({}, {}) -> {}", x, y, q),
        }
    }
}
impl fmt::Display for InputTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "InputTree[")?;

        for row in &self.quadrants {
            write!(f, "  [")?;
            for (i, q) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{q}")?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}

fn parse_input(input: &str) -> InputTree {
    let tree = input
        .lines()
        .into_iter()
        .map(|s| {
            s.chars()
                .into_iter()
                .map(|c| Quadrant::from(c))
                .collect::<Vec<_>>()
        })
        .collect();
    InputTree { quadrants: tree }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn parse_input_test() {
        let result = parse_input(INPUT);
        println!("{}", result);
        assert_eq!(count(&result, Quadrant::Start), 1);
        assert_eq!(count(&result, Quadrant::Splitter), 22);
        assert_eq!(count(&result, Quadrant::Empty), 217);
        assert_eq!(count(&result, Quadrant::Tachyon), 0);
    }

    fn count(tree: &InputTree, quadrant: Quadrant) -> usize {
        tree.quadrants
            .iter()
            .map(|row| row.iter().filter(|q| q.matches(quadrant)).count())
            .sum()
    }

    #[test]
    fn part1_test() {
        assert_eq!(Day07.part1(INPUT).unwrap(), "21");
    }

    #[test]
    fn part2_test() {
        assert_eq!(Day07.part2(INPUT).unwrap(), "40");
    }
}
