use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() {
    let file = File::open("aoc3-sample.txt").unwrap();
    let lines = BufReader::new(file).lines().map(<Result<String>>::unwrap);
    let mut sum = 0_usize;
    let mut buf = [false; 64];
    let offset_lower = 'a' as usize;
    let offset_upper = 'A' as usize;
    for line in lines {
        buf.fill(false);
        let len = line.len();
        let left = &line[..len/2];
        let right = &line[len/2..];
        left.chars().for_each(|c| {
            let ind = match c {
                'a'..='z' => (c as usize).saturating_sub(offset_lower),
                'A'..='Z' => (c as usize).saturating_sub(offset_upper) + 26,
                _ => unreachable!()
            };
            buf[ind] = true;
        });
        for c in right.chars() {
            let ind = match c {
                'a'..='z' => (c as usize).saturating_sub(offset_lower),
                'A'..='Z' => (c as usize).saturating_sub(offset_upper) + 26,
                _ => unreachable!()
            };
            if buf[ind] {
                sum += ind + 1;
                break;
            }
        }
    }
    println!("{sum}");
}