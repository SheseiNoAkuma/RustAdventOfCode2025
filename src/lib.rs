use std::collections::BTreeMap;

pub type AocResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait Day {
    fn name(&self) -> &'static str; // es. "day01"
    fn part1(&self, input: &str) -> AocResult<String>;
    fn part2(&self, input: &str) -> AocResult<String>;
}

pub mod days;

pub fn registry() -> BTreeMap<u32, Box<dyn Day>> {
    days::registry()
}