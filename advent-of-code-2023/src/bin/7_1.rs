use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::mem;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Five(String),
    Four(String),
    FullHouse(String),
    Three(String),
    Two(String),
    One(String),
    HighCard(String),
}

impl Hand {
    pub fn discriminant(&self) -> u8 {
        use Hand::*;
        match self {
            Five(_) => 7,
            Four(_) => 6,
            FullHouse(_) => 5,
            Three(_) => 4,
            Two(_) => 3,
            One(_) => 2,
            HighCard(_) => 1,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Hand::*;

        if mem::discriminant(self) != mem::discriminant(other) {
            // mem::discriminant doesn't impl `PartialOrd`
            self.discriminant().cmp(&other.discriminant())
        } else {
            match (self, other) {
                (Five(a), Five(b))
                | (Four(a), Four(b))
                | (FullHouse(a), FullHouse(b))
                | (Three(a), Three(b))
                | (Two(a), Two(b))
                | (One(a), One(b))
                | (HighCard(a), HighCard(b)) => a.cmp(b),
                _ => unreachable!(),
            }
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counts = HashMap::new();
        for c in s.chars() {
            let num = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '2'..='9' => c.to_digit(10).unwrap(),
                _ => unreachable!("{c}"),
            };
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut pairs = counts.into_iter().collect::<Vec<_>>();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

        // replace hands to sort by lexicographical order
        let s = s
            .to_string()
            .replace('A', "E")
            .replace('K', "D")
            .replace('Q', "C")
            .replace('J', "B")
            .replace('T', "A");
        let hand = match pairs.as_slice() {
            [(_, 5)] => Hand::Five(s),
            [(_, 4), (_, 1)] => Hand::Four(s),
            [(_, 3), (_, 2)] => Hand::FullHouse(s),
            [(_, 3), (_, 1), (_, 1)] => Hand::Three(s),
            [(_, 2), (_, 2), (_, 1)] => Hand::Two(s),
            [(_, 2), (_, 1), (_, 1), (_, 1)] => Hand::One(s),
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => Hand::HighCard(s),
            _ => unreachable!(),
        };

        Ok(hand)
    }
}

/// https://adventofcode.com/2023/day/7
/// Day 7: Camel Cards
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut hands = Vec::new();

    for line in lines {
        let line = line?;
        let (hand_str, score) = {
            let line = line.split(' ').map(|s| s.trim()).collect::<Vec<_>>();
            (line[0], line[1].parse::<usize>().unwrap())
        };
        let hand = Hand::from_str(hand_str).unwrap();
        hands.push((hand, score));
    }

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));
    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum::<usize>();

    Ok(winnings)
}

fn main() {
    println!("{}", solve("./input/day7_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day7_1.sample.txt").unwrap(), 6440);
    }
}
