use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use crate::common::{Result, SillyStringError};
use regex::Regex;

const STACK_COUNT: usize = 9;

#[derive(Debug, Default, Clone)]
struct Warehouse {
    stacks: [Vec<char>; STACK_COUNT],
}

impl Warehouse {
    pub fn create_9000(file: &File) -> Result<Self> {
        let mut stream = BufReader::new(file);
        let mut warehouse = Self::default();
        warehouse.load_stacks(&mut stream)?;
        warehouse.read_and_apply_moves(&mut stream)?;
        Ok(warehouse)
    }
    pub fn create_9001(file: &File) -> Result<Self> {
        let mut stream = BufReader::new(file);
        let mut warehouse = Self::default();
        warehouse.load_stacks(&mut stream)?;
        warehouse.read_and_apply_moves2(&mut stream)?;
        Ok(warehouse)
    }
    fn load_stacks(&mut self, stream: &mut BufReader<&File>) -> Result<()> {
        let pattern = Regex::new(r"(?:(?:(?:\[|\s)(.)(?:\]|\s))(?:\s|\n))")?;
        let mut stacks_data = Vec::default();
        stream.read_until(49, &mut stacks_data)?;
        let stacks_data = String::from_utf8(stacks_data)?;
        let mut stack_index = 0;
        for group in pattern.captures_iter(&stacks_data) {
            let id = group.get(1).unwrap().as_str().as_bytes()[0];
            if id != 32 {
                self.stacks[stack_index].push(id.into());
            }
            stack_index = (stack_index + 1) % STACK_COUNT;
        }
        for i in 0..STACK_COUNT {
            self.stacks[i].reverse();
        }
        Ok(())
    }
    fn read_and_apply_moves(&mut self, stream: &mut BufReader<&File>) -> Result<()> {
        let mut text = String::new();
        let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
        stream.read_to_string(&mut text)?;
        for capture in pattern.captures_iter(&text) {
            let count = capture.get(1).unwrap().as_str().parse()?;
            let from: usize = capture.get(2).unwrap().as_str().parse()?;
            let to: usize = capture.get(3).unwrap().as_str().parse()?;
            for _ in 0..count {
                let el = self.stacks[from - 1]
                    .pop()
                    .ok_or(SillyStringError::new(&"No more element in stack"))?;
                self.stacks[to - 1].push(el);
            }
        }
        Ok(())
    }
    fn read_and_apply_moves2(&mut self, stream: &mut BufReader<&File>) -> Result<()> {
        let mut text = String::new();
        let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
        stream.read_to_string(&mut text)?;
        for capture in pattern.captures_iter(&text) {
            let count: usize = capture.get(1).unwrap().as_str().parse()?;
            let from: usize = capture.get(2).unwrap().as_str().parse()?;
            let to: usize = capture.get(3).unwrap().as_str().parse()?;
            let stack_size = self.stacks[from - 1].len();
            let mut els: Vec<char> = self.stacks[from - 1]
                .drain(stack_size - count..stack_size)
                .collect();
            self.stacks[to - 1].append(&mut els);
        }
        Ok(())
    }
    pub fn print_last_line(&self) {
        for i in 0..STACK_COUNT {
            print!("{}", self.stacks[i].last().unwrap_or(&' '))
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..STACK_COUNT {
            for el in self.stacks[i].iter() {
                f.write_str(el.to_string().as_str())?;
                f.write_str(" ")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub fn challenge() {
    let input_file = File::open("inputs/day5.txt").unwrap();
    let warehouse = Warehouse::create_9000(&input_file).unwrap();
    warehouse.print_last_line();
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day5.txt").unwrap();
    let warehouse = Warehouse::create_9001(&input_file).unwrap();
    warehouse.print_last_line();
}
