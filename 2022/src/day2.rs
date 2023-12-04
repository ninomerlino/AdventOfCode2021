use crate::common::{Result, SillyStringError};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug,Default)]
pub enum Hand {
    #[default]
    Rock,
    Scissor,
    Paper,
}

impl Hand {
    pub fn challenge(&self, other: &Hand) -> u32 {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Scissor) => 6,
            (Hand::Rock, Hand::Paper) => 0,
            (Hand::Scissor, Hand::Rock) => 0,
            (Hand::Scissor, Hand::Scissor) => 3,
            (Hand::Scissor, Hand::Paper) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Paper, Hand::Scissor) => 0,
            (Hand::Paper, Hand::Paper) => 3,
        }
    }
    pub fn value(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Scissor => 3,
            Hand::Paper => 2,
        }
    }
    pub fn complementary_hand(
        s: &str,
        adversary: &Hand,
    ) -> std::result::Result<Self, SillyStringError> {
        match (s, adversary) {
            ("X", Hand::Rock) => Ok(Hand::Scissor),
            ("X", Hand::Paper) => Ok(Hand::Rock),
            ("X", Hand::Scissor) => Ok(Hand::Paper),
            ("Y", Hand::Rock) => Ok(Hand::Rock),
            ("Y", Hand::Paper) => Ok(Hand::Paper),
            ("Y", Hand::Scissor) => Ok(Hand::Scissor),
            ("Z", Hand::Rock) => Ok(Hand::Paper),
            ("Z", Hand::Paper) => Ok(Hand::Scissor),
            ("Z", Hand::Scissor) => Ok(Hand::Rock),
            _ => Err(SillyStringError::new(&format!(
                "Cannot uknown order : |{}| with |{:?}|",
                s, adversary
            ))),
        }
    }
}

impl FromStr for Hand {
    type Err = SillyStringError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(SillyStringError::new(&format!(
                "Cannot parse from : |{}|",
                s
            ))),
        }
    }
}

#[derive(Debug, Default)]
pub struct Round {
    yours: Hand,
    adversary: Hand,
}

impl Round {
    pub fn points(&self) -> u32 {
        self.yours.value() + self.yours.challenge(&self.adversary)
    }
    pub fn load_rounds(file: &File) -> Result<Vec<Round>> {
        let file = BufReader::new(file);
        let mut rounds = Vec::new();
        for line in file.lines() {
            let line = line?;
            let mut sub = line.split(' ');
            let round = Round {
                adversary: sub
                    .next()
                    .ok_or(SillyStringError::new(&"Missing adversary hand"))?
                    .parse()?,
                yours: sub
                    .next()
                    .ok_or(SillyStringError::new(&"Missing your hand"))?
                    .parse()?,
            };

            rounds.push(round);
        }
        Ok(rounds)
    }
    pub fn load_rounds_2(file: &File) -> Result<Vec<Round>> {
        let file = BufReader::new(file);
        let mut rounds = Vec::new();
        for line in file.lines() {
            let line = line?;
            let mut sub = line.split(' ');
            let mut round = Round {
                adversary: sub
                    .next()
                    .ok_or(SillyStringError::new(&"Missing adversary hand"))?
                    .parse()?,
                ..Default::default()
            };

            let order = sub.next().ok_or(SillyStringError::new(&"Missing order"))?;
            round.yours = Hand::complementary_hand(order, &round.adversary)?;
            rounds.push(round);
        }
        Ok(rounds)
    }
}

pub fn challenge() {
    let input_file = File::open("inputs/day2.txt").unwrap();
    let rounds = Round::load_rounds(&input_file).unwrap();
    println!(
        "Total score = {}",
        rounds.iter().map(|el| el.points()).sum::<u32>()
    );
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day2.txt").unwrap();
    let rounds = Round::load_rounds_2(&input_file).unwrap();
    println!(
        "Total score = {}",
        rounds.iter().map(|el| el.points()).sum::<u32>()
    );
}
