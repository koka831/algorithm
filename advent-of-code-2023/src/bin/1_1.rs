use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// https://adventofcode.com/2023/day/1
/// Day 1: Trebuchet?!
fn solve(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(&file).lines();
    let mut count = 0;

    let parse = |c: char| c as usize - '0' as usize;

    for line in lines {
        let line = line.unwrap();
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rfind(char::is_ascii_digit).unwrap();

        count += parse(first) * 10 + parse(last);
    }

    count
}

fn main() {
    println!("{}", solve("./input/day1_1.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day1_1.sample.txt"), 142);
    }
}
