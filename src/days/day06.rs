use crate::{AocResult, Day};

pub struct Day06;
impl Day for Day06 {
    fn name(&self) -> &'static str {
        "day06"
    }

    fn part1(&self, input: &str) -> AocResult<String> {
        let problems = parse_input(input);
        let result = problems.iter().map(|p| p.resolve()).sum::<u64>();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> AocResult<String> {
        let problems = parse_input(input);
        let result = problems
            .iter()
            .map(|p| p.convert_to_cephalopod_math().resolve())
            .sum::<u64>();
        Ok(result.to_string())
    }
}

fn parse_input(input: &str) -> Vec<Problem> {
    // 1) righe non vuote, preservando spazi interni/finali
    let mut lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return vec![];
    }

    // 2) ultima riga = operatori
    let ops_line = lines.pop().unwrap();
    let num_lines = lines;

    // 3) posizioni operatori e operatori
    let mut op_positions: Vec<usize> = Vec::new();
    let mut ops: Vec<Operator> = Vec::new();

    for (i, b) in ops_line.as_bytes().iter().enumerate() {
        match *b as char {
            '*' | '+' => {
                op_positions.push(i);
                // safe perché è un singolo byte ASCII
                let s = &ops_line[i..i + 1];
                ops.push(Operator::from(s));
            }
            _ => {}
        }
    }

    let mut widths: Vec<usize> = Vec::with_capacity(op_positions.len());
    for w in op_positions.windows(2) {
        let a = w[0];
        let b = w[1];
        widths.push(b - a - 1);
    }

    let last_width = if widths.is_empty() {
        0
    } else {
        *widths.last().unwrap()
    };
    widths.push(last_width);

    fn take_segment(line: &str, start: usize, width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let bytes = line.as_bytes();
        let end = start.saturating_add(width);

        let mut out = if start >= bytes.len() {
            String::new()
        } else {
            let e = end.min(bytes.len());
            // ASCII-only → slicing by bytes ok
            line[start..e].to_string()
        };

        // pad a destra se la riga è corta (così la "cella" rimane di width fissa)
        if out.len() < width {
            out.push_str(&" ".repeat(width - out.len()));
        }
        out
    }

    (0..ops.len())
        .map(|col| {
            let start = op_positions[col];
            let width = widths[col];

            let numbers = num_lines
                .iter()
                .map(|line| take_segment(line, start, width))
                .collect::<Vec<_>>();

            Problem {
                numbers,
                operator: ops[col].clone(),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    Multiply,
    Add,
}
impl Operator {
    fn apply(&self, a: &u64, b: &u64) -> u64 {
        match self {
            Self::Multiply => a * b,
            Self::Add => a + b,
        }
    }
}
impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "*" => Self::Multiply,
            "+" => Self::Add,
            _ => panic!("invalid operator: {}", value),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
struct Problem {
    numbers: Vec<String>,
    operator: Operator,
}
impl Problem {
    fn resolve(&self) -> u64 {
        let result = self
            .numbers
            .iter()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .reduce(|acc, i| self.operator.apply(&acc, &i));

        result.unwrap_or(0)
    }

    fn convert_to_cephalopod_math(&self) -> Problem {
        let num_as_str: Vec<_> = self.numbers.iter().map(|n| n.to_string()).collect();
        let max_length = num_as_str.iter().map(|s| s.len()).max().unwrap_or(0);

        let cephalopod_numbers: Vec<_> = (0..max_length)
            .map(|i| {
                num_as_str
                    .iter()
                    .map(|s| s.chars().nth(i).map(|c| vec![c]).unwrap_or_default())
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .map(|chars| chars.iter().collect::<String>())
            .collect();

        Problem {
            numbers: cephalopod_numbers,
            operator: self.operator.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn parse_input_test() {
        let p1: Problem = Problem {
            numbers: vec!["123".to_string(), " 45".to_string(), "  6".to_string()],
            operator: Operator::Multiply,
        };
        let p2: Problem = Problem {
            numbers: vec!["328".to_string(), "64 ".to_string(), "98 ".to_string()],
            operator: Operator::Add,
        };
        let p3: Problem = Problem {
            numbers: vec![" 51".to_string(), "387".to_string(), "215".to_string()],
            operator: Operator::Multiply,
        };
        let p4: Problem = Problem {
            numbers: vec!["64 ".to_string(), "23 ".to_string(), "314".to_string()],
            operator: Operator::Add,
        };
        assert_eq!(parse_input(INPUT), vec![p1, p2, p3, p4]);
    }

    #[test]
    fn convert_to_cephalopod_math_test() {
        let p1: Problem = Problem {
            numbers: vec!["64 ".to_string(), "23 ".to_string(), "314".to_string()],
            operator: Operator::Multiply,
        };
        assert_eq!(
            p1.convert_to_cephalopod_math(),
            Problem {
                numbers: vec!["623".to_string(), "431".to_string(), "  4".to_string()],
                operator: Operator::Multiply,
            }
        );
        let p2 = Problem {
            numbers: vec![" 51".to_string(), "387".to_string(), "215".to_string()],
            operator: Operator::Add,
        };
        assert_eq!(
            p2.convert_to_cephalopod_math(),
            Problem {
                numbers: vec![" 32".to_string(), "581".to_string(), "175".to_string()],
                operator: Operator::Add,
            }
        );
    }

    /**
    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401
    the grand total is 33210 + 490 + 4243455 + 401 = 4277556
    */
    #[test]
    fn part1_test() {
        assert_eq!(Day06.part1(INPUT).unwrap(), "4277556");
    }

    /**
    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544
    Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.
    */
    #[test]
    fn part2_test() {
        assert_eq!(Day06.part2(INPUT).unwrap(), "3263827");
    }
}
