use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/5
/// Day 5: If You Give A Seed A Fertilizer
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();

    let first = lines.next().unwrap().unwrap();
    let seeds = first
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    lines.next().unwrap().unwrap();

    let mut mapping = Vec::new();
    let mut current = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match line {
            l if l.contains(':') => {}
            l if l.is_empty() => {
                mapping.push(current.clone());
                current.clear();
            }
            l => {
                let map = l
                    .split_whitespace()
                    .map(|c| c.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                current.push((map[0], map[1], map[2]));
            }
        }
    }
    if !current.is_empty() {
        mapping.push(current.clone());
    }

    let mut min = std::usize::MAX;

    for ss in seeds.chunks(2) {
        for seed in ss[0]..(ss[0] + ss[1]) {
            let mut v = seed;
            for m in &mapping {
                match reduction(v, m) {
                    Some(redux) => v = redux,
                    None => continue,
                }
            }

            min = min.min(v);
        }
    }

    Ok(min)
}

fn reduction(v: usize, map: &[(usize, usize, usize)]) -> Option<usize> {
    for &(dest, src, len) in map {
        if (src..(src + len)).contains(&v) {
            return Some(dest + v - src);
        }
    }

    None
}

fn main() {
    println!("{}", solve("./input/day5.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day5.sample.txt").unwrap(), 46);
    }
}
