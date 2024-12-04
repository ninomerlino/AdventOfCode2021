use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    time::Instant,
};

use regex::Regex;

fn load_day1(file: &File) -> (Vec<u32>, Vec<u32>) {
    let (list1, list2): (Vec<u32>, Vec<u32>) = BufReader::new(file)
        .lines()
        .map(|line| {
            let splited: Vec<u32> = line
                .unwrap()
                .split("   ")
                .map(|word| word.parse::<u32>().unwrap())
                .collect();
            (splited[0], splited[1])
        })
        .unzip();
    (list1, list2)
}

fn solve_part1_day1(input: (Vec<u32>, Vec<u32>)) -> String {
    let mut list1 = input.0;
    list1.sort();
    let mut list2 = input.1;
    list2.sort();
    let sum: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    sum.to_string()
}

fn solve_part2_day1(input: (Vec<u32>, Vec<u32>)) -> String {
    let list1 = input.0;
    let frequencies = input
        .1
        .into_iter()
        .fold(BTreeMap::<u32, u64>::new(), |mut map, el| {
            *map.entry(el).or_insert(0) += 1;
            map
        });

    let sum: u64 = list1
        .iter()
        .map(|el| (*el as u64) * (frequencies.get(el).unwrap_or(&0)))
        .sum();
    sum.to_string()
}

#[test]
fn part1_day1() {
    let file = File::open("./input/day1.txt").unwrap();
    let input = load_day1(&file);
    let result = solve_part1_day1(input);
    println!("Result for day 1 part 1 is {result}");
}

#[test]
fn part2_day1() {
    let file = File::open("./input/day1.txt").unwrap();
    let input = load_day1(&file);
    let result = solve_part2_day1(input);
    println!("Result for day 1 part 2 is {result}");
}

fn load_day2(file: &File) -> Vec<Vec<i32>> {
    BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split(' ')
                .map(|word| word.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_valid(report: &Vec<i32>) -> bool {
    let direction = if report[0] < report[1] { 1 } else { -1 };
    for i in 1..report.len() {
        let delta = (report[i] - report[i - 1]) * direction;
        if !(1..=3).contains(&delta) {
            return false;
        }
    }
    true
}

fn is_report_almost_valid(report: &Vec<i32>) -> bool {
    if is_report_valid(report) {
        return true;
    }

    for i in 0..report.len() {
        let partial_copy = report
            .iter()
            .enumerate()
            .filter_map(|(idx, el)| if idx != i { Some(*el) } else { None })
            .collect();
        if is_report_valid(&partial_copy) {
            return true;
        }
    }
    false
}

fn solve_part1_day2(input: Vec<Vec<i32>>) -> String {
    input
        .into_iter()
        .filter(is_report_valid)
        .count()
        .to_string()
}

fn solve_part2_day2(input: Vec<Vec<i32>>) -> String {
    input
        .into_iter()
        .filter(is_report_almost_valid)
        .count()
        .to_string()
}

#[test]
fn part1_day2() {
    let file = File::open("./input/day2.txt").unwrap();
    let input = load_day2(&file);
    let result = solve_part1_day2(input);
    println!("Result for day 2 part 1 is {result}");
}

#[test]
fn part2_day2() {
    let file = File::open("./input/day2.txt").unwrap();
    let input = load_day2(&file);
    let result = solve_part2_day2(input);
    println!("Result for day 2 part 2 is {result}");
}

fn load_day3(file: &File) -> String {
    //mul\(([0-9]{1,3}),([0-9]{1,3})\)
    let mut input = String::new();
    BufReader::new(file).read_to_string(&mut input).unwrap();
    input
}

fn solve_part1_day3(input: &str) -> String {
    //mul\(([0-9]{1,3}),([0-9]{1,3})\)
    let pattern = Regex::new(r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)"#).unwrap();
    pattern
        .captures_iter(input)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * c.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn solve_part2_day3(input: &str) -> String {
    //mul\(([0-9]{1,3}),([0-9]{1,3})\)
    let pattern =
        Regex::new(r#"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(?<E>do\(\))|(?<D>don't\(\))"#).unwrap();

    let mut result = 0;
    let mut enabled = true;
    for capture in pattern.captures_iter(input) {
        if let Some(_) = capture.name("E") {
            enabled = true;
        } else if let Some(_) = capture.name("D") {
            enabled = false;
        } else {
            if enabled {
                result += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
            }
        }
    }
    result.to_string()
}
#[test]
fn part1_day3() {
    let file = File::open("./input/day3.txt").unwrap();
    let input = load_day3(&file);
    let result = solve_part1_day3(&input);
    println!("Result for day 3 part 1 is {result}");
}

#[test]
fn part2_day3() {
    let file = File::open("./input/day3.txt").unwrap();
    let input = load_day3(&file);
    let result = solve_part2_day3(&input);
    println!("Result for day 3 part 2 is {result}");
}

fn main() {}
