use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() -> Result<()> {
    let file = File::open("aoc3.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut sum = 0_usize;
    let mut buf = [0_u8; 64];
    
    let offset_lower = 'a' as usize;
    let offset_upper = 'A' as usize;
    
    while let Some(line) = lines.next() {
        buf.fill(0);
        line?.chars().for_each(|c| {
            let ind = match c {
                'a'..='z' => (c as usize).saturating_sub(offset_lower),
                'A'..='Z' => (c as usize).saturating_sub(offset_upper) + 26,
                _ => unreachable!()
            };
            buf[ind] |= 1;
        });

        let line = lines.next().unwrap();
        line?.chars().for_each(|c| {
            let ind = match c {
                'a'..='z' => (c as usize).saturating_sub(offset_lower),
                'A'..='Z' => (c as usize).saturating_sub(offset_upper) + 26,
                _ => unreachable!()
            };
            buf[ind] |= 1 << 1;
        });

        let line = lines.next().unwrap();
        for c in line?.chars() {
            let ind = match c {
                'a'..='z' => (c as usize).saturating_sub(offset_lower),
                'A'..='Z' => (c as usize).saturating_sub(offset_upper) + 26,
                _ => unreachable!()
            };
            if buf[ind] == 3 {
                sum += ind + 1;
                break;
            }
        }
    }
    println!("{sum}");
    Ok(())
}