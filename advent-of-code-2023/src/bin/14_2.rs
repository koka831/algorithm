#![feature(let_chains)]
#![allow(unused)]
#![allow(clippy::needless_range_loop)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/14
/// Day 14: Parabolic Reflector Dish
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();
    let mut g = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        g.push(line.trim().chars().collect::<Vec<_>>());
    }
    let score = rotate(&g).iter().map(|line| tilt(line)).sum();

    Ok(score)
}

// score per line
fn tilt(line: &[char]) -> usize {
    let len = line.len();
    // left most
    let mut rock = 0;
    // piled rounded rocks on the left-most cube-shaped rock
    let mut piled = 0;
    let mut score = 0;
    for (i, c) in line.iter().enumerate() {
        match c {
            '#' => {
                rock = i + 1;
                piled = 0;
            }
            '.' => {}
            'O' => {
                let pos = rock + piled;
                score += len - pos;
                piled += 1;
            }
            _ => unreachable!(),
        }
    }

    score
}

fn rotate(g: &[Vec<char>]) -> Vec<Vec<char>> {
    let (xlen, ylen) = (g[0].len(), g.len());
    (0..xlen)
        .map(|x| (0..ylen).map(|y| g[y][x]).collect())
        .collect()
}

fn main() {
    println!("{}", solve("./input/day14.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        // assert_eq!(solve("./input/day14.sample.txt").unwrap(), 64);
    }
}
