use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/20
/// Day 20: Pulse Propagation
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();
    while let Some(Ok(line)) = lines.next() {}

    Ok(0)
}

fn main() {
    println!("{}", solve("./input/day20.sample.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day20.sample1.txt").unwrap(), 32000000);
        assert_eq!(solve("./input/day20.sample2.txt").unwrap(), 11687500);
    }
}
