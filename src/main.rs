use std::{fs, path::PathBuf};

use advent_of_code::{registry, AocResult};

fn main() -> AocResult<()> {
    let mut args = std::env::args().skip(1);

    let day: u32 = args
        .next()
        .ok_or("Usage: cargo run -- <day> <part> [input_path]")?
        .parse()?;

    let part: u32 = args
        .next()
        .ok_or("Usage: cargo run -- <day> <part> [input_path]")?
        .parse()?;

    let input_path: PathBuf = match args.next() {
        Some(p) => p.into(),
        None => format!("inputs/day{:02}.txt", day).into(),
    };

    let input = fs::read_to_string(&input_path)?;

    let reg = registry();
    let solver = reg.get(&day).ok_or("Unknown day")?;

    let out = match part {
        1 => solver.part1(&input)?,
        2 => solver.part2(&input)?,
        _ => return Err("Part must be 1 or 2".into()),
    };

    println!("{}", out);
    Ok(())
}
