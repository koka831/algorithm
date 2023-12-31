use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/15
/// Day 15: Lens Library
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let line = std::io::read_to_string(file).unwrap();

    let mut map = vec![vec![]; 256];
    for seq in line.split(',').map(|s| s.trim()) {
        let len = seq.len();
        let (key, op, val) = {
            if seq.contains('=') {
                let key = &seq[..len - 2];
                let op = &seq[len - 2..len - 1];
                let val = &seq[len - 1..];

                (key, op, val)
            } else {
                let key = &seq[..len - 1];
                let op = &seq[len - 1..len];
                let val = &seq[len..];

                (key, op, val)
            }
        };
        let hashed = hash(key);
        match op {
            "=" => {
                let num = val.parse::<usize>().unwrap();
                if let Some(pos) = map[hashed].iter().position(|e: &(&str, usize)| e.0 == key) {
                    map[hashed][pos] = (key, num);
                } else {
                    map[hashed].push((key, num));
                }
            }
            "-" => {
                if let Some(pos) = map[hashed].iter().position(|e| e.0 == key) {
                    map[hashed].remove(pos);
                }
            }
            _ => unreachable!(),
        }
    }

    let power = map
        .iter()
        .enumerate()
        .map(|(box_id, b)| {
            b.iter()
                .enumerate()
                .map(|(slot, (_, focal))| (box_id + 1) * (slot + 1) * focal)
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(power)
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
        assert_eq!(solve("./input/day15.sample.txt").unwrap(), 145);
    }
}
