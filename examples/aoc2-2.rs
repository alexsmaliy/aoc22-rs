use std::fs::File;
use std::io::{BufReader, BufRead, Result};

enum RPS {
    Rock,
    Paper,
    Scissors,
}

struct Pair(RPS, RPS);

impl TryFrom<(&str, &str)> for Pair {
    type Error = ();

    fn try_from((them, you): (&str, &str)) -> std::result::Result<Self, Self::Error> {
        use RPS::*;
        if them.eq("A") && you.eq("X") {
            Ok(Pair(Rock, Scissors))
        } else if them.eq("A") && you.eq("Y") {
            Ok(Pair(Rock, Rock))
        } else if them.eq("A") && you.eq("Z") {
            Ok(Pair(Rock, Paper))
        } else if them.eq("B") && you.eq("X") {
            Ok(Pair(Paper, Rock))
        } else if them.eq("B") && you.eq("Y") {
            Ok(Pair(Paper, Paper))
        } else if them.eq("B") && you.eq("Z") {
            Ok(Pair(Paper, Scissors))
        } else if them.eq("C") && you.eq("X") {
            Ok(Pair(Scissors, Paper))
        } else if them.eq("C") && you.eq("Y") {
            Ok(Pair(Scissors, Scissors))
        } else if them.eq("C") && you.eq("Z") {
            Ok(Pair(Scissors, Rock))
        } else {
            Err(())
        }
    }
}

impl Pair {
    pub fn score(&self) -> u32 {
        use RPS::*;
        match (&self.0, &self.1) {
            (Rock, Rock) => 1 + 3,
            (Rock, Paper) => 2 + 6,
            (Rock, Scissors) => 3 + 0,
            (Paper, Rock) => 1 + 0,
            (Paper, Paper) => 2 + 3,
            (Paper, Scissors) => 3 + 6,
            (Scissors, Rock) => 1 + 6,
            (Scissors, Paper) => 2 + 0,
            (Scissors, Scissors) => 3 + 3,
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
        acc += Pair::try_from((left, right)).unwrap().score();
    }
    println!("{acc}");
}