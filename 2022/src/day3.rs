use identity_hash::BuildIdentityHasher;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::common::{FastHashSet, Loadable, Result, SillyStringError};

#[derive(Debug)]
pub struct Backpack {
    comp1: String,
    comp2: String,
}

impl Backpack {
    pub fn new(s: &String) -> Result<Self> {
        let end = s.len();
        let middle = end / 2;
        let comp1 = s[0..middle].to_string();
        let comp2 = s[middle..end].to_string();
        if comp1.len() == comp2.len() {
            Ok(Backpack { comp1, comp2 })
        } else {
            Err(SillyStringError::new(&"two compartments have different sizes").into())
        }
    }
    pub fn item_priority(c: u8) -> Result<u32> {
        let value = c as u32;
        if (65..=90).contains(&value)  {
            Ok(value - 38)
        } else if (97..=122).contains(&value) {
            Ok(value - 96)
        } else {
            Err(SillyStringError::new(&"Invalid byte     found").into())
        }
    }
    pub fn find_duplicate_item(&self) -> Result<u8> {
        let mut already_read_items = FastHashSet::with_hasher(BuildIdentityHasher::default());
        for byte in self.comp1.as_bytes() {
            already_read_items.insert(*byte);
        }
        for byte in self.comp2.as_bytes() {
            if already_read_items.contains(byte) {
                return Ok(*byte);
            }
        }
        Err(SillyStringError::new(&"No duplicate found").into())
    }
    pub fn find_duplicate_items(&self, other: &Backpack) -> FastHashSet<u8> {
        let mut already_read_items = FastHashSet::with_hasher(BuildIdentityHasher::default());
        let mut duplicates = FastHashSet::with_hasher(BuildIdentityHasher::default());
        for byte in self.comp1.as_bytes() {
            already_read_items.insert(*byte);
        }
        for byte in self.comp2.as_bytes() {
            already_read_items.insert(*byte);
        }
        for byte in other.comp1.as_bytes() {
            if already_read_items.contains(byte) {
                duplicates.insert(*byte);
            }
        }
        for byte in other.comp2.as_bytes() {
            if already_read_items.contains(byte) {
                duplicates.insert(*byte);
            }
        }
        duplicates
    }
    pub fn find_duplicate_items_hashset(&self, other: &FastHashSet<u8>) -> FastHashSet<u8> {
        let mut duplicates = other.clone();
        duplicates.retain(|el| self.comp1.as_bytes().contains(el) || self.comp2.as_bytes().contains(el));
        duplicates
    }
    fn find_group_badge(group: [&Backpack; 3]) -> Result<u8> {
        let duplicates = group[0].find_duplicate_items(group[1]);
        let duplicates = group[2].find_duplicate_items_hashset(&duplicates);
        let badge =  duplicates.iter().next().ok_or(SillyStringError::new(&"Invalid duplicates"))?;
        Ok(*badge)
    }
}

impl Loadable for Backpack {
    fn load(file: &File) -> Result<Vec<Self>> {
        let file = BufReader::new(file);
        let mut backpacks = Vec::new();
        for line in file.lines() {
            let line = line?;
            let backpack = Backpack::new(&line)?;
            backpacks.push(backpack);
        }
        Ok(backpacks)
    }
}

pub fn challenge() {
    let input_file = File::open("inputs/day3.txt").unwrap();
    let backpacks = Backpack::load(&input_file).unwrap();
    let mut total_priority = 0;
    for backpack in backpacks {
        let duplicate = backpack.find_duplicate_item().unwrap();
        total_priority += Backpack::item_priority(duplicate).unwrap();
    }
    println!("Total priority = {}", total_priority);
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day3.txt").unwrap();
    let backpacks = Backpack::load(&input_file).unwrap();
    let mut iter = backpacks.iter().peekable();
    let mut total_priority = 0;
    loop {
        if iter.peek().is_none() {
            break;
        }
        let group: [&Backpack; 3] = [
            iter.next()
                .ok_or(SillyStringError::new(&"Group item 0 not found"))
                .unwrap(),
            iter.next()
                .ok_or(SillyStringError::new(&"Group item 1 not found"))
                .unwrap(),
            iter.next()
                .ok_or(SillyStringError::new(&"Group item 2 not found"))
                .unwrap(),
        ];
        let badge = Backpack::find_group_badge(group).unwrap();
        total_priority += Backpack::item_priority(badge).unwrap();
    }
    println!("Total priority = {}", total_priority);
}
