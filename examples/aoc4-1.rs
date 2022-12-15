use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() -> Result<()> {
    let file = File::open("aoc4.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut count = 0_u16;

    for line in lines {
        let line = line?;
        let line: Vec<_> = line.split(',')
                                   .take(2)
                                   .map(
                                    |substr| {
                                        let mut toks = substr.split('-');
                                        let from = toks.next()?.parse::<u16>().ok()?;
                                        let to = toks.next()?.parse::<u16>().ok()?;
                                        Some(from..=to)
                                    })
                                   .map(Option::unwrap).collect();
        if (line[0].start() <= line[1].start() && line[0].end() >= line[1].end())
           ||
           (line[1].start() <= line[0].start() && line[1].end() >= line[0].end()) {
            count += 1;
        }
    }
    println!("{count}");
    Ok(())
}
