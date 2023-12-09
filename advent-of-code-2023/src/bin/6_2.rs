use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/6
/// Day 6: Wait For It
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let input = std::io::read_to_string(file)?;
    let lines = input.trim().split('\n').collect::<Vec<_>>();

    let times = parse(lines[0]);
    let dists = parse(lines[1]);

    let mut count = 1;
    for (limit, dist) in times.iter().zip(dists.iter()) {
        let mut ways = 0;
        for i in 0..*limit {
            if (limit - i) * i > *dist {
                ways += 1;
            }
        }

        count *= ways;
    }

    Ok(count)
}

fn parse(s: &str) -> Vec<usize> {
    s.replace(' ', "")
        .split(':')
        .skip(1)
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    println!("{}", solve("./input/day6_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day6_1.sample.txt").unwrap(), 71503);
    }
}
