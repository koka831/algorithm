use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/4
/// Day 4: Scratchcards
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let input = std::io::read_to_string(file)?;
    let lines = input.trim().split('\n').collect::<Vec<_>>();

    let mut cards = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (winner, mine) = line[line.find(':').unwrap() + 1..]
            .split_once('|')
            .map(|(winner, mine)| (parse(winner), parse(mine)))
            .unwrap();

        let point = mine.iter().filter(|m| winner.contains(m)).count();
        for j in 0..point {
            cards[i + j + 1] += cards[i];
        }
    }

    let total = cards.iter().sum::<usize>();

    Ok(total)
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
        assert_eq!(solve("./input/day4_1.sample.txt").unwrap(), 30);
    }
}
