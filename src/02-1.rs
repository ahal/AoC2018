use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

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
    println!("Day 1 - Part 2");
    let lines = read_lines("src/data/day-02.txt");
    let mut two = 0;
    let mut three = 0;

    for line in lines {
        println!("{}", line);
        let mut counts = HashMap::new();
        for c in line.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }
        println!("{:?}", counts);
        if counts.values().any(|&v| v == 2) {
            two +=1;
        }
        if counts.values().any(|&v| v == 3) {
            three +=1;
        }
    }
    let checksum = two * three;
    println!("Checksum: {}", checksum);
}
