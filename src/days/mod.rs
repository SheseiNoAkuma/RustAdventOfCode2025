use std::collections::BTreeMap;

use crate::Day;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn registry() -> BTreeMap<u32, Box<dyn Day>> {
    let mut m: BTreeMap<u32, Box<dyn Day>> = BTreeMap::new();
    m.insert(1, Box::new(day01::Day01));
    m.insert(2, Box::new(day02::Day02));
    m.insert(3, Box::new(day03::Day03));
    m.insert(4, Box::new(day04::Day04));
    m.insert(5, Box::new(day05::Day05));
    m
}
