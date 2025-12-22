use std::collections::BTreeMap;

use crate::Day;

pub mod day01;

pub fn registry() -> BTreeMap<u32, Box<dyn Day>> {
    let mut m: BTreeMap<u32, Box<dyn Day>> = BTreeMap::new();
    m.insert(1, Box::new(day01::Day01));
    m
}