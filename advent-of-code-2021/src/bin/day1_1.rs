/// https://adventofcode.com/2021/day/1
/// Day 1: Sonar Sweep
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input/day1_1.txt").unwrap();
    let mut lines = BufReader::new(&file).lines();
    let mut count = 0;
    let mut prev: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines {
        let depth = line.unwrap().parse().unwrap();
        if prev < depth {
            count += 1;
        }
        prev = depth;
    }

    println!("{}", count);
}
