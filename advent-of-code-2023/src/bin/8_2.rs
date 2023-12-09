use std::collections::HashMap;
use std::fs::File;
use std::io::{self};

/// https://adventofcode.com/2023/day/8
/// Day 8: Haunted Wasteland
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let input = std::io::read_to_string(&file).unwrap();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    let instructions = input[0].chars().cycle();
    let mut network = HashMap::new();
    for line in &input[2..] {
        let line = line
            .replace('=', ",")
            .replace(['(', ')'], "")
            .split(',')
            .map(|node| node.trim().to_owned())
            .collect::<Vec<_>>();

        network.insert(line[0].clone(), [line[1].clone(), line[2].clone()]);
    }

    let start = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();

    let steps = |start: &str| -> usize {
        let mut step = 0;
        let mut current = start;

        for direction in instructions.clone() {
            step += 1;
            let direction = if direction == 'L' { 0 } else { 1 };
            current = &network.get(current).unwrap()[direction];
            if current.ends_with('Z') {
                break;
            }
        }

        step
    };

    let mut count = 1;
    for s in start {
        count = lcm(count, steps(s));
    }

    Ok(count)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    println!("{}", solve("./input/day8_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day8_3.sample.txt").unwrap(), 6);
    }
}
