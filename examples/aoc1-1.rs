use std::fs::File;
use std::io::{BufReader, BufRead, self};

fn main() {
    let file = File::open("./aoc1.txt").unwrap();
    let lines = BufReader::new(file).lines().map(<io::Result<String>>::unwrap);
    let mut groups: Vec<Vec<u32>> = vec![vec![]];
    lines.for_each(|line| {
        if line.is_empty() {
            groups.push(vec![])
        } else {
            let len = groups.len().saturating_sub(1);
            groups[len].push(line.parse().unwrap());
        }
    });
    let max: u32 = groups.into_iter().map(|group| group.into_iter().sum()).max().unwrap();
    println!("{max}");
}