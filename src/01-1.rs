use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File not found!");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("Day 1 - Part 1");
    let lines = read_lines("src/data/day-01-1.txt");
    let mut freq: i32 = 0;
    for line in lines {
        let sign = &line[0..1];
        let num: i32 = line[1..].parse().unwrap();
        if sign == "+" {
            freq += num;
        } else {
            freq -= num;
        }
    }
    println!("Frequency: {}", freq);
}
