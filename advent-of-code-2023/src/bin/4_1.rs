use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/4
/// Day 4: Scratchcards
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut count = 0;

    for line in lines {
        let line = line?;
        let (winner, mine) = line[line.find(':').unwrap() + 1..]
            .split_once('|')
            .map(|(winner, mine)| (parse(winner), parse(mine)))
            .unwrap();

        let point = mine.iter().filter(|m| winner.contains(m)).count();
        count += 2usize.pow(point as u32) / 2;
    }

    Ok(count)
}

fn parse(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    println!("{}", solve("./input/day4_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day4_1.sample.txt").unwrap(), 13);
    }
}
