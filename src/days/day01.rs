use crate::{AocResult, Day};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Day01;

static STARTING_POSITION: i32 = 50;
static TOTAL_POSITIONS: i32 = 100;

impl Day for Day01 {
    fn name(&self) -> &'static str {
        "day01"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let rotations =
            parse_input(input)?
                .iter()
                .fold((STARTING_POSITION, 0), |(pos, counter), r| {
                    let new_pos: i32 = (match r.direction {
                        Direction::Left => pos - r.length,
                        Direction::Right => pos + r.length,
                    }) % TOTAL_POSITIONS;

                    let new_counter = if new_pos == 0 { counter + 1 } else { counter };
                    (new_pos, new_counter)
                });

        Ok(rotations.1.to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        println!("{}", input);
        todo!("not yet implemented")
    }
}

fn parse_input(input: &str) -> AocResult<Vec<Rotation>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| Rotation::try_from(line.trim()).map_err(|e| e.into()))
        .collect()
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug, PartialEq)]
struct ParseDirectionError(char);

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(ParseDirectionError(c)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    length: i32,
}
impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}{}", self.direction, self.length))
    }
}

#[derive(Debug, PartialEq)]
struct ParseRotationError(String);

impl Display for ParseRotationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ParseRotationError {}

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid direction char: {}", self.0)
    }
}
impl Error for ParseDirectionError {}

impl TryFrom<&str> for Rotation {
    type Error = ParseRotationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = match value.chars().next() {
            Some(c) => Direction::try_from(c)
                .map_err(|_| ParseRotationError("can't parse direction".to_string()))?,
            None => return Err(ParseRotationError("empty input".to_string())),
        };

        let length = value
            .get(1..) // skip first char
            .ok_or(ParseRotationError("missing length".to_string()))?
            .parse::<i32>()
            .map_err(|_| ParseRotationError("can't parse rotation length".to_string()))?;

        Ok(Rotation { direction, length })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_direction_l() {
        assert_eq!(Direction::try_from('L').unwrap(), Direction::Left);
    }
    #[test]
    fn parse_direction_r() {
        assert_eq!(Direction::try_from('R').unwrap(), Direction::Right);
    }
    #[test]
    fn parse_direction_error() {
        assert_eq!(Direction::try_from('W'), Err(ParseDirectionError('W')));
    }

    #[test]
    fn parse_rotation_empty() {
        assert_eq!(
            Rotation::try_from(""),
            Err(ParseRotationError(String::from("empty input")))
        );
    }
    #[test]
    fn parse_rotation_direction_only() {
        assert_eq!(
            Rotation::try_from("L"),
            Err(ParseRotationError(String::from(
                "can't parse rotation length"
            )))
        );
    }

    #[test]
    fn parse_rotation_invalid_number() {
        assert_eq!(
            Rotation::try_from("L_NotANumber"),
            Err(ParseRotationError(String::from(
                "can't parse rotation length"
            )))
        );
    }

    #[test]
    fn parse_rotation_valid() {
        assert_eq!(
            Rotation::try_from("L10").unwrap(),
            Rotation {
                direction: Direction::Left,
                length: 10
            }
        );
        assert_eq!(
            Rotation::try_from("R22").unwrap(),
            Rotation {
                direction: Direction::Right,
                length: 22
            }
        );
    }

    #[test]
    fn parse_input_ok() {
        assert_eq!(
            parse_input("L10\nR22").unwrap(),
            vec![
                Rotation {
                    direction: Direction::Left,
                    length: 10
                },
                Rotation {
                    direction: Direction::Right,
                    length: 22
                }
            ]
        );
    }

    #[test]
    fn part1_counts_lines() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(Day01.part1(input).unwrap(), "3");
    }
}
