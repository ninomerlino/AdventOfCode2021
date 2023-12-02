use std::collections::HashMap;

use regex::Regex;

struct ReduceState {
    sum: u64,
    last: u64,
    flush: bool,
}

impl Default for ReduceState {
    fn default() -> Self {
        Self {
            sum: 0,
            last: 0,
            flush: true,
        }
    }
}

fn is_valid_char(c: &char) -> bool {
    c.is_ascii_digit() || *c == '\n'
}

fn reduce_char(mut s: ReduceState, c: char) -> ReduceState {
    if s.flush {
        let digit = c.to_digit(10).unwrap() as u64;
        s.sum += digit * 10;
        s.last = digit;
        s.flush = false;
    } else if c == '\n' {
        s.sum += s.last;
        s.flush = true;
    } else {
        let digit = c.to_digit(10).unwrap() as u64;
        s.last = digit;
    }
    s
}

#[aoc(day1, part1)]
pub fn p1(input: &str) -> u64 {
    let s = input
        .chars()
        .filter(is_valid_char)
        .fold(ReduceState::default(), reduce_char);
    s.sum + s.last
}

#[aoc(day1, part2)]
pub fn p2(input: &str) -> u64 {
	let input = input
	.replace("one","o1e")
	.replace("two","t2o")
	.replace("three","t3e")
	.replace("four","f4r")
	.replace("five","f5e")
	.replace("six","s6x")
	.replace("seven","s7n")
	.replace("eight","e8t")
	.replace("nine","n9e");
    let s = input
        .chars()
        .filter(is_valid_char)
        .fold(ReduceState::default(), reduce_char);
    s.sum + s.last
}

const DAY2_PART1: &'static str = r"([0-9]+)\s(blue|red|green)";

#[aoc(day2, part1)]
pub fn p3(input: &str) -> u64 {
    let pattern = Regex::new(DAY2_PART1).unwrap();
    let mut limits = HashMap::new();
    limits.insert("red",12);
    limits.insert("green",13);
    limits.insert("blue",14);
    let mut count = 0;
    for (id, line) in input.lines().enumerate(){
        count += id + 1;
        for c in pattern.captures_iter(line){
            if limits[c.get(2).unwrap().as_str()] < c.get(1).unwrap().as_str().parse().unwrap() {
                count -= id + 1;
                break;
            }
        }
    }
    
    count as u64
}

#[aoc(day2, part2)]
pub fn p4(input: &str) -> u64 {
    let pattern = Regex::new(DAY2_PART1).unwrap();
    let mut limits: HashMap<&str, u64> = HashMap::new();
    let mut total_power = 0_u64;
    for line in input.lines(){
        limits.insert("red",0);
        limits.insert("green",0);
        limits.insert("blue",0);
        for c in pattern.captures_iter(line){
            if let Some(limit) = limits.get_mut(c.get(2).unwrap().as_str()) {
                let value = c.get(1).unwrap().as_str().parse().unwrap();
                *limit = (*limit).max(value);
            }
        }
        total_power += limits.values().fold(1,|acc,b|acc*b);
    }
    total_power as u64
}