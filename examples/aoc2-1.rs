use std::fs::File;
use std::io::{BufReader, BufRead, Result};

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn value(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn outcome(&self, other: &RPS) -> u32 {
        use RPS::*;
        match (self, other) {
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Scissors) => 0,
            (Paper, Rock) => 6,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            _ => 3,
        }
    }

    pub fn score(them: &RPS, you: &RPS) -> u32 {
        you.value() + you.outcome(them)
    }
}

impl TryFrom<&str> for RPS {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        use RPS::*;
        if value.eq("A") || value.eq("X") {
            Ok(Rock)
        } else if value.eq("B") || value.eq("Y") {
            Ok(Paper)
        } else if value.eq("C") || value.eq("Z") {
            Ok(Scissors)
        } else {
            Err(())
        }
    }
}

fn main() {
    let file = File::open("aoc2.txt").unwrap();
    let lines = BufReader::new(file).lines().map(<Result<String>>::unwrap);
    let mut acc: u32 = 0;
    for line in lines {
        let mut toks = line.split_ascii_whitespace().map(|tok| tok.trim());
        let left = toks.next().unwrap();
        let right = toks.next().unwrap();
        acc += RPS::score(
            &RPS::try_from(left).unwrap(),
            &RPS::try_from(right).unwrap(),
        );
    }
    println!("{acc}");
}