use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::iter::Peekable;

enum FileSystemNode {
    AdventFile {
        name: String,
        size: i32,
    },
    AdventDir {
        name: String,
        children: Children,
        size: i32,
    },
}

impl Display for FileSystemNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use FileSystemNode::*;
        match self {
            AdventFile { name, size } => {
                f.write_str(&format!("\
                {{\
                    \"name\": \"{}\", \
                    \"size\": {}\
                }}", name, size))
            },
            AdventDir { name, size, .. } => {
                f.write_str(&format!("{{\"name\": \"{}\", \"size\": {}}}", name, size))
            },
        }
    }
}

#[derive(Default)]
struct Children(HashMap<String, FileSystemNode>);

impl Display for Children {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = self.0.values();
        let b = a.map(<FileSystemNode as ToString>::to_string);
        let c: Vec<_> = b.collect();
        let d = c.join(",");
        f.write_str(&format!("[{}]", d))
    }
}

fn get_next_input_chunk<I>(input: &mut Peekable<I>) -> Vec<String>
where I: Iterator<Item = Result<String>> {
    let mut next_input: Vec<String> = vec![];
    let next_line = input.next();
    if next_line.is_some() {
        next_input.push(next_line.unwrap().unwrap());
        while input.peek().as_deref().map_or(false, |line| line.as_deref().map_or(false, |line| !line.starts_with("$"))) {
            next_input.push(input.next().unwrap().unwrap());
        }
    }
    next_input
}

fn process_ls<I>(children: &mut HashMap<String, FileSystemNode>, size: &mut i32, output: &mut Peekable<I>)
where I: Iterator<Item = Result<String>> {
    let command = get_next_input_chunk(output);
    for line in command.iter().skip(1) {
        if line.starts_with("dir") {
            let dirname = line.rsplit("dir ").next().unwrap();
            if !children.contains_key(dirname) {
                children.insert(dirname.into(), FileSystemNode::AdventDir {
                    name: dirname.into(),
                    children: Children::default(),
                    size: 0_i32
                });
            }
        } else {
            let mut line = line.split(" ");
            let filesize = line.next().unwrap().parse::<i32>().unwrap();
            let filename = line.next().unwrap();
            children.insert(
                filename.into(),
                FileSystemNode::AdventFile { name: filename.into(), size: filesize }
            );
            *size += filesize;
        }
    }
}

impl FileSystemNode {
    fn explore<I>(&mut self, output: &mut Peekable<I>) -> i32
    where I: Iterator<Item = Result<String>> {
        use FileSystemNode::*;
        match self {
            AdventFile {..} => panic!("can't treat a file as a directory!"),
            AdventDir { children, size, ..} => {
                let Children(children) = children;
                let old_size = *size;
                loop {
                    let maybe_upcoming = output.peek();
                    let upcoming = match maybe_upcoming.as_deref() {
                        None => None,
                        Some(Result::Err(_)) => None,
                        Some(Result::Ok(x)) => Some(x),
                    };
                    match upcoming {
                        None => {
                            return *size - old_size;
                        },
                        Some(command) if command.starts_with("$ ls") => {
                            process_ls(children, size, output);
                        },
                        Some(command) if command.starts_with("$ cd ..") => {
                            let _ = get_next_input_chunk(output);
                            return *size - old_size;
                        },
                        Some(command) if command.starts_with("$ cd ") => {
                            let dirname = command.rsplit("$ cd ").next().unwrap();
                            match children.get_mut(dirname) {
                                Some(dir) => {
                                    let _ = get_next_input_chunk(output);
                                    let size_delta = dir.explore(output);
                                    *size += size_delta;
                                },
                                None => {
                                    panic!("Tried to cd into {dirname}, which we have not yet seen!")
                                },
                            }
                        },
                        Some(command) => {
                            panic!("Unexpected input: {command}");
                        }
                    }
                }
            }
        }
    }
}

fn find_dir_to_delete(dir: &FileSystemNode, min_size: i32) -> Option<&FileSystemNode> {
    fn visit<'a, 'b>(dir: &'a FileSystemNode, q: &'b mut VecDeque<&'a FileSystemNode>) {
        match dir {
            FileSystemNode::AdventDir {children, ..} => {
                q.push_front(&*dir);
                for (_, child) in &children.0 {
                    match child {
                        FileSystemNode::AdventDir {..} => visit(child, q),
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    let mut q = std::collections::VecDeque::<&FileSystemNode>::default();
    match dir {
        FileSystemNode::AdventFile {..} => panic!(),
        FileSystemNode::AdventDir {..} => visit(dir, &mut q),
    }
    // q.iter().for_each(|x| println!("{x}"));
    let a = q.iter();
    let b = a.filter(|dir| match dir {
        FileSystemNode::AdventDir {size, ..} => {
            *size >= min_size
        },
        _ => false,
    });
    let c = b.reduce(|acc, e| match (acc, e) {
        (FileSystemNode::AdventDir { size: best_size, .. }, FileSystemNode::AdventDir { size: curr_size, .. }) => {
            if curr_size < best_size { e } else { acc }
        },
        _ => acc,
    });
    c.map(|x| *x)
}

fn main() -> Result<()> {
    let file = File::open("aoc7.txt")?;
    let mut lines = BufReader::new(file).lines().skip(1).peekable();
    let mut root = FileSystemNode::AdventDir { name: "/".into(), children: Children::default(), size: 0 };
    root.explore(&mut lines);
    let total = 70_000_000_i32;
    let used = if let FileSystemNode::AdventDir { size, .. } = &root { *size } else { panic!() };
    let free = total - used;
    let needed = 30_000_000_i32;
    let to_delete = needed - free;
    let dir = find_dir_to_delete(&root, to_delete).unwrap();
    println!("need to delete {to_delete} -- {dir}");
    Ok(())
}