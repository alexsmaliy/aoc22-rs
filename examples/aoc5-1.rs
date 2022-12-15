use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn main() -> Result<()> {
    let file = File::open("aoc5.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut stacks: Vec<Vec<char>> = vec![];
    parse_stacks(&mut lines, &mut stacks);
    let mut instructions: Vec<[u8; 3]> = vec![];
    parse_instructions(&mut lines, &mut instructions);
    for instruction in instructions {
        execute(&mut stacks, instruction);
    }
    let tops: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("{tops}");
    Ok(())
}

fn execute(stacks: &mut Vec<Vec<char>>, instruction: [u8; 3]) -> Option<()> {
    let [quantity, source, dest] = instruction;
    for _ in 0..quantity {
        let moved = stacks.get_mut(usize::from(source))?.pop()?;
        stacks.get_mut(usize::from(dest))?.push(moved);
    }
    Some(())
}

fn parse_instructions(lines: &mut impl Iterator<Item = Result<String>>, instructions: &mut Vec<[u8; 3]>) -> Option<()> {
    while let Some(Ok(line)) = lines.next() {
        let mut nums = line.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty());
        instructions.push([
            nums.next()?.parse::<u8>().ok()?,
            nums.next()?.parse::<u8>().ok()?.saturating_sub(1),
            nums.next()?.parse::<u8>().ok()?.saturating_sub(1),
        ]);
    }
    Some(())
}

fn parse_stacks(lines: &mut impl Iterator<Item = Result<String>>, stacks: &mut Vec<Vec<char>>) -> Option<()> {
    let line = lines.next()?.ok()?;
    let mut cursor = 0_usize;
    let mut index = 0_usize;
    while cursor < line.len() {
        stacks.push(vec![]);
        let tok = &line[cursor..cursor+3];
        if !tok.trim().is_empty() {
            let c = tok.chars().nth(1)?;
            stacks[index].push(c);
        }
        index += 1;
        cursor += 4;
    }
    while let Some(Ok(line)) = lines.next() {
        let mut cursor = 0_usize;
        let mut index = 0_usize;
        if line.trim().is_empty() {
            break;
        } else {
            while cursor < line.len() {
                let tok = &line[cursor..cursor+3];
                if !tok.trim().is_empty() {
                    let c = tok.chars().nth(1)?;
                    stacks[index].push(c);
                }
                index += 1;
                cursor += 4;
            }
        }
    }
    for stack in stacks.iter_mut() {
        *stack = stack.iter().rev().copied().collect();
    }
    Some(())
}