#![feature(let_chains)]
#![allow(unused)]
#![allow(clippy::needless_range_loop)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/
/// Day
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();
    while let Some(Ok(line)) = lines.next() {}

    Ok(0)
}

fn main() {
    // println!("{}", solve("./input/day.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        // assert_eq!(solve("./input/day.sample.txt").unwrap(), 405);
    }
}
