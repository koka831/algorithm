/// https://adventofcode.com/2021/day/1
/// Day 1: Sonar Sweep Part Two
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input/day1_2.txt").unwrap();
    let depths = BufReader::new(&file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Vec<usize>>();

    let window_depths = depths
        .windows(3)
        .map(|ds| ds.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    let count = window_depths.windows(2).filter(|&v| v[0] < v[1]).count();
    println!("{}", count);
}
