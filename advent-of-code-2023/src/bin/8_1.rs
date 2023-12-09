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

    let mut count = 0;
    let mut current = "AAA";

    for direction in instructions {
        count += 1;
        let direction = if direction == 'L' { 0 } else { 1 };

        current = &network.get(current).unwrap()[direction];
        if current == "ZZZ" {
            break;
        }
    }

    Ok(count)
}

fn main() {
    println!("{}", solve("./input/day8_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day8_1.sample.txt").unwrap(), 2);
        assert_eq!(solve("./input/day8_2.sample.txt").unwrap(), 6);
    }
}
