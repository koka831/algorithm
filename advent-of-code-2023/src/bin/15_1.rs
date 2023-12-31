use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/15
/// Day 15: Lens Library
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let line = std::io::read_to_string(file).unwrap();

    let result = line.split(',').map(|s| hash(s.trim())).sum();
    Ok(result)
}

fn hash(s: &str) -> usize {
    let mut current = 0;
    for c in s.chars() {
        current = (current + (c as u8) as usize) * 17;
        current %= 256;
    }

    current
}

fn main() {
    println!("{}", solve("./input/day15.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day15.sample.txt").unwrap(), 1320);
    }
}
