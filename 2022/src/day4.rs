use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::common::{Result, SillyStringError};

#[derive(Debug, Clone, Copy)]
pub struct Sector {
    start: u16,
    end: u16,
}

enum ReadingStatus {
    First,
    Second(Sector),
}

impl Sector {
    pub fn new(s: &str) -> Result<Self> {
        let mut values: Vec<u16> = Vec::new();
        for el in s.split('-') {
            values.push(el.parse()?)
        }
        if values.len() == 2 {
            Ok(Self {
                start: values[0],
                end: values[1],
            })
        } else {
            Err(SillyStringError::new(&"Invalid sector count").into())
        }
    }
    pub fn load_pairs(file: &mut File) -> Result<Vec<(Sector, Sector)>> {
        let file = BufReader::new(file);
        let mut pairs = Vec::new();
        let mut status = ReadingStatus::First;
        for line in file.lines() {
            let line = line?;
            for sector in line.split(',') {
                match status {
                    ReadingStatus::First => {
                        status = ReadingStatus::Second(Sector::new(sector)?);
                    }
                    ReadingStatus::Second(first) => {
                        pairs.push((first, Sector::new(sector)?));
                        status = ReadingStatus::First;
                    }
                }
            }
        }
        Ok(pairs)
    }
    pub fn check_fully_contain(s1: &Sector, s2: &Sector) -> bool {
        (s1.start <= s2.start && s2.end <= s1.end) || (s2.start <= s1.start && s1.end <= s2.end)
    }
    pub fn check_if_overlaps(s1: &Sector, s2: &Sector) -> bool {
        (s2.start >= s1.start && s2.start <= s1.end)
            || (s2.end >= s1.start && s2.end <= s1.end)
            || Self::check_fully_contain(s1, s2)
    }
}

pub fn challenge() {
    let mut input_file = File::open("inputs/day4.txt").unwrap();
    let mut pairs = Sector::load_pairs(&mut input_file).unwrap();
    pairs.retain(|(s1, s2)| Sector::check_fully_contain(s1, s2));
    println!("number of fully contained sector is {}", pairs.len());
}

pub fn challenge_2() {
    let mut input_file = File::open("inputs/day4.txt").unwrap();
    let mut pairs = Sector::load_pairs(&mut input_file).unwrap();
    pairs.retain(|(s1, s2)| Sector::check_if_overlaps(s1, s2));
    println!("number of fully contained sector is {}", pairs.len());
}
