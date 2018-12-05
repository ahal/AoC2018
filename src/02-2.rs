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

fn similar_chars(string_a: &String, string_b: &String) -> Vec<char>
{
    let mut similar_chars: Vec<char> = vec![];
    let mut b_chars = string_b.chars();
    for c in string_a.chars() {
        if c == b_chars.next().unwrap() {
            similar_chars.push(c);
        }
    }
    similar_chars
}

fn main() {
    println!("Day 2 - Part 2");
    let lines_a = read_lines("src/data/day-02.txt");
    let lines_b = lines_a.to_vec();
    let mut id: String = String::from("");

    'found: for line_a in &lines_a {
        for line_b in &lines_b {
            if line_a == line_b {
                continue;
            }

            let chars = similar_chars(line_a, line_b);
            if chars.len() == line_a.len() - 1 {
                id = chars.into_iter().collect();
                break 'found;
            }
        }
    }
    println!("Id: {}", id);
}
