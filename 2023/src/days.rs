use ahash::RandomState as ARandomState;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::{Range, RangeInclusive},
};

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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Pos(i64, i64);

impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i64(self.1 + self.0 * 140)
    }
}

#[aoc_generator(day3, part1)]
pub fn g5(input: &str) -> (String, HashSet<Pos, ARandomState>) {
    let mut valid_positions: HashSet<Pos, ARandomState> = HashSet::default();
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
    (input.to_owned(), valid_positions)
}

#[aoc_generator(day3, part2)]
pub fn g6(input: &str) -> (String, HashMap<Pos, Pos, ARandomState>) {
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
    (input.to_owned(), valid_positions)
}

#[aoc(day3, part1)]
pub fn p5((text, valid_positions): &(String, HashSet<Pos, ARandomState>)) -> u64 {
    let mut sum = 0;
    let mut is_valid = false;
    let mut buffer = String::default();

    for (row, line) in text.lines().enumerate() {
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

#[aoc(day3, part2)]
pub fn p6((input, gear_positions): &(String, HashMap<Pos, Pos, ARandomState>)) -> u64 {
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

    gear_friends
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum()
}

enum P7State {
    Start,
    WinningNumbers,
    NormalNumbers,
}

#[derive(Debug, Clone, Default)]
pub struct Deck {
    id: usize,
    winning_numbers: HashSet<u64, ARandomState>,
    numbers: Vec<u64>,
}

impl Deck {
    fn new(id: usize, line: &str) -> Self {
        let mut numbers: Vec<u64> = Vec::default();
        let mut winning_numbers: HashSet<u64, ARandomState> = HashSet::default();
        let mut state = P7State::Start;

        for word in line.split_whitespace() {
            match state {
                P7State::Start => {
                    if word.contains(":") {
                        state = P7State::WinningNumbers;
                    }
                }
                P7State::WinningNumbers => {
                    if word == "|" {
                        state = P7State::NormalNumbers;
                    } else {
                        winning_numbers.insert(word.parse().unwrap());
                    }
                }
                P7State::NormalNumbers => {
                    numbers.push(word.parse().unwrap());
                }
            }
        }

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn points(&self) -> u64 {
        self.numbers
            .iter()
            .filter_map(|n| {
                if self.winning_numbers.contains(n) {
                    Some(1_u64)
                } else {
                    None
                }
            })
            .reduce(|acc, _| acc * 2)
            .unwrap_or(0)
    }

    fn pairs(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
    }
}

#[aoc_generator(day4)]
pub fn g7(input: &str) -> Vec<Deck> {
    input
        .lines()
        .enumerate()
        .map(|(id, l)| Deck::new(id, l))
        .collect()
}

#[aoc(day4, part1)]
pub fn p7(input: &Vec<Deck>) -> u64 {
    input.iter().map(|d| d.points()).sum()
}

#[aoc(day4, part2)]
pub fn p8(input: &Vec<Deck>) -> u64 {
    let mut mult_values: HashMap<usize, u64, ARandomState> = HashMap::default();
    let mut total = 0;
    for deck in input {
        let mult = mult_values.get(&deck.id).copied().unwrap_or(1);
        let pairs = deck.pairs();
        for target_id in (deck.id + 1)..=(deck.id + pairs) {
            mult_values
                .entry(target_id)
                .and_modify(|x| *x += mult)
                .or_insert(mult + 1);
        }
        total += mult;
    }
    total
}

/*#[aoc(day5, part1)]
pub fn p9(input: &str) -> u64 {
    todo!() /* day 5 boring as hell */
}

#[aoc(day5, part2)]
pub fn p10(input: &str) -> u64 {
    todo!()
}*/

#[derive(Debug)]
pub struct Race {
    total_time: u64,
    min_winning_distance: u64,
}

impl Race {
    pub fn get_winning_strats_count(&self) -> u64 {
        let delta = ((self.total_time.pow(2) - 4 * self.min_winning_distance) as f64).sqrt();
        let min = (self.total_time as f64 - delta) / 2.0;
        let max = (self.total_time as f64 + delta) / 2.0;
        (max.floor() - min.ceil() + 1.0) as u64
    }
}

#[aoc_generator(day6, part1)]
pub fn g11(input: &str) -> Vec<Race> {
    let mut lines = input.lines().map(|l| l.split_whitespace());
    let mut races = lines.next().unwrap().zip(lines.next().unwrap());
    races.next(); //skip labels
    races
        .map(|(time, distance)| Race {
            total_time: time.parse().unwrap(),
            min_winning_distance: distance.parse::<u64>().unwrap() + 1,
        })
        .collect()
}

#[aoc_generator(day6, part2)]
pub fn g12(input: &str) -> Race {
    let mut lines = input.lines().map(|l| l.split_whitespace());
    let mut time = lines.next().unwrap();
    let mut distance = lines.next().unwrap();
    time.next(); //skip labels
    distance.next();
    Race {
        total_time: time.collect::<String>().parse().unwrap(),
        min_winning_distance: distance.collect::<String>().parse::<u64>().unwrap() + 1,
    }
}

#[aoc(day6, part1)]
pub fn p11(input: &Vec<Race>) -> u64 {
    input
        .iter()
        .map(|r| r.get_winning_strats_count())
        .reduce(|acc, n| acc * n)
        .unwrap()
}

#[aoc(day6, part2)]
pub fn p12(input: &Race) -> u64 {
    input.get_winning_strats_count()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandScore {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandScore {
    pub fn new(card_freq: &[u8; 13]) -> HandScore {
        let mut max = 0;
        let mut pair_count = 0;
        for freq in card_freq {
            max = max.max(*freq);
            if *freq == 2 {
                pair_count += 1;
            }
        }

        match (max, pair_count) {
            (5, _) => HandScore::FiveOfAKind,
            (4, _) => HandScore::FourOfAKind,
            (3, 1) => HandScore::FullHouse,
            (3, 0) => HandScore::ThreeOfAKind,
            (2, 2) => HandScore::TwoPair,
            (2, 1) => HandScore::OnePair,
            (1, 0) => HandScore::HighCard,
            _ => unreachable!(),
        }
    }

    pub fn new_joker(card_freq: &[u8; 13]) -> HandScore {
        let mut max = 0;
        let mut pair_count = 0;
        let jokers = card_freq[0];
        for freq in card_freq {
            max = max.max(*freq);
            if *freq == 2 {
                pair_count += 1;
            }
        }
        match (max, pair_count, jokers) {
            (5, _, _) => HandScore::FiveOfAKind,
            (4, 0, 4) => HandScore::FiveOfAKind,
            (4, _, 1) => HandScore::FiveOfAKind,
            (4, _, 0) => HandScore::FourOfAKind,
            (3, 1, 3) => HandScore::FiveOfAKind,
            (3, 1, 2) => HandScore::FiveOfAKind,
            (3, 1, 0) => HandScore::FullHouse,
            (3, 0, 3) => HandScore::FourOfAKind,
            (3, 0, 1) => HandScore::FourOfAKind,
            (3, 0, 0) => HandScore::ThreeOfAKind,
            (2, 2, 2) => HandScore::FourOfAKind,
            (2, 2, 1) => HandScore::FullHouse,
            (2, 2, 0) => HandScore::TwoPair,
            (2, 1, 2) => HandScore::ThreeOfAKind,
            (2, 1, 1) => HandScore::ThreeOfAKind,
            (2, 1, 0) => HandScore::OnePair,
            (1, 0, 1) => HandScore::OnePair,
            (1, 0, 0) => HandScore::HighCard,
            x => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    raw: String,
    bid: u64,
    score: HandScore,
}

impl Hand {
    #[inline]
    fn card_index(c: char) -> usize {
        match c {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        }
    }

    fn card_index_joker(c: char) -> usize {
        match c {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => unreachable!(),
        }
    }

    pub fn new(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let card_freq = cards.chars().fold([0_u8; 13], |mut freq, c| {
            freq[Self::card_index(c)] += 1;
            freq
        });

        Self {
            raw: cards.to_owned(),
            bid: bid.parse().unwrap(),
            score: HandScore::new(&card_freq),
        }
    }

    pub fn new_joker(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let card_freq = cards.chars().fold([0_u8; 13], |mut freq, c| {
            freq[Self::card_index_joker(c)] += 1;
            freq
        });

        Self {
            raw: cards.to_owned(),
            bid: bid.parse().unwrap(),
            score: HandScore::new_joker(&card_freq),
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self
                .raw
                .chars()
                .map(Self::card_index)
                .zip(other.raw.chars().map(Self::card_index))
                .map(|(s, o)| s.cmp(&o))
                .find(|p| *p != Ordering::Equal)
                .unwrap(),
            ord => ord,
        }
    }

    fn compare_joker(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self
                .raw
                .chars()
                .map(Self::card_index_joker)
                .zip(other.raw.chars().map(Self::card_index_joker))
                .map(|(s, o)| s.cmp(&o))
                .find(|p| *p != Ordering::Equal)
                .unwrap(),
            ord => ord,
        }
    }
}

#[aoc_generator(day7, part1)]
pub fn g13(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

#[aoc_generator(day7, part2)]
pub fn g14(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new_joker).collect()
}

#[aoc(day7, part1)]
pub fn p13(input: &Vec<Hand>) -> u64 {
    let mut input = input.to_vec();
    input.sort_by(Hand::compare);
    input
        .iter()
        .enumerate()
        .map(|(inx, card)| card.bid * (inx as u64 + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn p14(input: &Vec<Hand>) -> u64 {
    let mut input: Vec<Hand> = input.to_vec();
    input.sort_by(Hand::compare_joker);
    input
        .iter()
        .enumerate()
        .map(|(inx, card)| card.bid * (inx as u64 + 1))
        .sum()
}

#[aoc_generator(day9)]
pub fn g15(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn predict_next_value(history: &[i64]) -> i64 {
    let mut last_values: Vec<i64> = Vec::new();
    let mut history: Vec<i64> = history.to_owned();

    let mut all_zero = false;
    let mut last = history.len() - 1;

    while !all_zero || last == 0 {
        all_zero = true;
        last_values.push(history[last]);
        for index in 0..last {
            history[index] =  history[index + 1] - history[index];
            all_zero &= history[index] == 0;
        }
        last -= 1;
    }
    last_values.iter().sum()
}

pub fn predict_prev_value(history: &[i64]) -> i64 {
    let mut last_values: Vec<i64> = Vec::new();
    let mut history: Vec<i64> = history.to_owned();

    let mut all_zero = false;
    let mut last = history.len() - 1;

    while !all_zero || last == 0 {
        all_zero = true;
        last_values.push(history[0]);
        for index in 0..last {
            history[index] =  history[index + 1] - history[index];
            all_zero &= history[index] == 0;
        }
        last -= 1;
    }
    last_values.iter().rev().fold(0,|acc,el|*el - acc)
}

#[aoc(day9, part1)]
pub fn p15(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|vec|predict_next_value(vec)).sum()
}

#[aoc(day9, part2)]
pub fn p16(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|vec|predict_prev_value(vec)).sum()
}
