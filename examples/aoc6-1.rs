use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() -> Result<()> {
    let file = File::open("aoc6.txt")?;
    let mut line = String::new();
    let mut deque = VecDeque::<char>::new();
    let mut map = HashMap::<char, u8>::new();
    if BufReader::new(file).read_line(&mut line).is_err() { panic!() }
    let mut chars = line.chars();
    let len = line.len();
    for _ in 0_usize..4 {
        let c = chars.next().unwrap();
        deque.push_back(c);
        map.entry(c).and_modify(|e| {*e += 1} ).or_insert(1);
    }
    if map.len() == 4 {
        println!("4");
        return Ok(());
    }
    for ind in 4_usize..len {
        let oldest = deque.pop_front().unwrap();
        let oldest_count = map.get(&oldest).unwrap();
        if *oldest_count > 1 {
            map.insert(oldest, oldest_count.saturating_sub(1));
        } else {
            map.remove(&oldest);
        }
        let c = chars.next().unwrap();
        deque.push_back(c);
        map.entry(c).and_modify(|e| {*e += 1} ).or_insert(1);
        if map.len() == 4 {
            println!("{}", ind + 1);
            return Ok(());
        }
    }
    Ok(())
}