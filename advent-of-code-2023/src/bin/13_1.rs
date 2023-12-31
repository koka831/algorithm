#![feature(let_chains)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/13
/// Day 13: Point of Incidence
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines().peekable();
    let mut score = 0;

    while lines.peek().is_some() {
        let mut g = Vec::new();
        while let Some(Ok(line)) = lines.next() && !line.is_empty() {
            g.push(line.chars().collect::<Vec<_>>());
        }

        let s = reflection(&g)
            .or_else(|| reflection(&rotate(&g)).map(|y| y * 100))
            .unwrap_or(0);
        score += s;
    }

    Ok(score)
}

fn reflection(g: &[Vec<char>]) -> Option<usize> {
    let xlen = g[0].len();
    for x in 0..xlen - 1 {
        let (mut l, mut r) = (x, x + 1);
        while g.iter().all(|n| n[l] == n[r]) {
            if l == 0 || r == xlen - 1 {
                return Some(x + 1);
            }

            l -= 1;
            r += 1;
        }
    }

    None
}

fn rotate(g: &[Vec<char>]) -> Vec<Vec<char>> {
    let (xlen, ylen) = (g[0].len(), g.len());
    (0..xlen)
        .map(|x| (0..ylen).map(|y| g[y][x]).collect())
        .collect()
}

fn main() {
    println!("{}", solve("./input/day13.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day13.sample.txt").unwrap(), 405);
    }
}
