use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    println!("FOOBAR");
    println!("{:?}", filename);
    let file = File::open(filename).expect("File not found!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("Day 1 - Part 1");
    let lines = read_lines("src/data/day-01-1.txt");
    for line in lines {
        println!("{:?}", line);
    }
}
