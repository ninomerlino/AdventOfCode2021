use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use ahash::RandomState as ARandomState;
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
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
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
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);
    let mut count = 0;
    for (id, line) in input.lines().enumerate() {
        count += id + 1;
        for c in pattern.captures_iter(line) {
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
    for line in input.lines() {
        limits.insert("red", 0);
        limits.insert("green", 0);
        limits.insert("blue", 0);
        for c in pattern.captures_iter(line) {
            if let Some(limit) = limits.get_mut(c.get(2).unwrap().as_str()) {
                let value = c.get(1).unwrap().as_str().parse().unwrap();
                *limit = (*limit).max(value);
            }
        }
        total_power += limits.values().fold(1, |acc, b| acc * b);
    }
    total_power as u64
}

#[derive(Debug,Default,PartialEq,Eq,Clone, Copy)]
struct Pos(i64,i64);

impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i64(self.1 + self.0*140)
    }
}

fn generate_valid_positions(input: &str) -> HashSet<Pos,ARandomState> {
    let mut valid_positions: HashSet<Pos,ARandomState> = HashSet::default();
    for (row, line) in input.lines().enumerate() {
        let row = row as i64;
        for (col, val) in line.chars().enumerate() {
            let col = col as i64;
            if val.is_ascii_digit() || val == '.' {
                continue;
            }
            for new_row in (row - 1)..=(row + 1) {
                for new_col in (col - 1)..=(col + 1) {
                    valid_positions.insert(Pos(new_row, new_col));
                }
            }
        }
    }
    valid_positions
}



#[aoc(day3, part1, merlino)]
pub fn p5(input: &str) -> u64 {
    let valid_positions = generate_valid_positions(input);

    let mut sum = 0;
    let mut is_valid = false;
    let mut buffer = String::default();

    for (row, line) in input.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            if val.is_ascii_digit() {
                buffer.push(val);
                is_valid |= valid_positions.contains(&Pos(row as i64, col as i64));
            } else {
                if is_valid {
                    sum += buffer.parse::<u64>().unwrap();
                    is_valid = false;
                }
                buffer.clear();
            }
        }
        if is_valid {
            sum += buffer.parse::<u64>().unwrap();
            is_valid = false;
        }
        buffer.clear();
    }
    sum
}

fn generate_gear_positions(input: &str) -> HashMap<Pos, Pos, ARandomState> {
    let mut valid_positions: HashMap<Pos, Pos, ARandomState> = HashMap::default();
    for (row, line) in input.lines().enumerate() {
        let row = row as i64;
        for (col, val) in line.chars().enumerate() {
            let col = col as i64;
            if val == '*' {
                for new_row in (row - 1)..=(row + 1) {
                    for new_col in (col - 1)..=(col + 1) {
                        valid_positions.insert(Pos(new_row, new_col), Pos(row, col));
                    }
                }
            }
        }
    }
    valid_positions
}

#[aoc(day3, part2, merlino)]
pub fn p6(input: &str) -> u64 {
    let gear_positions = generate_gear_positions(input);

    let mut gear_friends: HashMap<Pos, Vec<u64>, ARandomState> = HashMap::default();
    let mut close_gears: HashSet<Pos, ARandomState> = HashSet::default();
    let mut buffer = String::default();

    for (row, line) in input.lines().enumerate() {
        for (col, val) in line.chars().enumerate() {
            if val.is_ascii_digit() {
                buffer.push(val);
                if let Some(close_gear) = gear_positions.get(&Pos(row as i64, col as i64)) {
                    close_gears.insert(*close_gear);
                }
            } else {
                for gear in close_gears.iter() {
                    let num = buffer.parse::<u64>().unwrap();
                    gear_friends
                        .entry(*gear)
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);
                }
                close_gears.clear();
                buffer.clear();
            }
        }
        for gear in close_gears.iter() {
            let num = buffer.parse::<u64>().unwrap();
            gear_friends
                .entry(*gear)
                .and_modify(|v| v.push(num))
                .or_insert(vec![num]);
        }
        close_gears.clear();
        buffer.clear();
    }

    gear_friends.values().filter_map(|v| {
        if v.len() == 2 {
            Some(v[0] * v[1])
        } else {
            None
        }
    }).sum()
}