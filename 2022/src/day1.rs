use crate::common::{Loadable, Result};
use core::slice::Iter;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
pub struct Elf {
    total: u32,
    items: Vec<u32>,
}

impl Elf {
    pub fn total(&self) -> u32 {
        self.total
    }
    pub fn items(&self) -> Iter<'_, u32> {
        self.items.iter()
    }
}

impl Loadable for Elf {
    fn load(file: &File) -> Result<Vec<Elf>> {
        let mut elfs = Vec::new();
        let mut elf = Elf::default();
        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.is_empty() {
                elf.total = elf.items().sum();
                elfs.push(elf);
                elf = Elf::default();
            } else {
                elf.items.push(line.parse()?);
            }
        }
        Ok(elfs)
    }
}

pub fn challenge_1() {
    let input_file = File::open("inputs/day1.txt").unwrap();
    let elfs = Elf::load(&input_file).unwrap();
    println!("{:?}", elfs.iter().max_by_key(|el| el.total()))
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day1.txt").unwrap();
    let mut elfs = Elf::load(&input_file).unwrap();
    let mut total = 0;
    elfs.sort_by_key(|el| el.total);
    elfs.reverse();
    for e in &elfs[0..3] {
        total += e.total();
    }
    println!("{:?}", total);
}
