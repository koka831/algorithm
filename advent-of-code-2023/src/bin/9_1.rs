use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/9
/// Day 9: Mirage Maintenance
fn solve(path: &str) -> io::Result<isize> {
    let file = File::open(path)?;
    let input = std::io::read_to_string(file)?;
    let extrapolated = input.trim().split('\n').map(process).sum::<isize>();

    Ok(extrapolated)
}

fn process(line: &str) -> isize {
    let history = line
        .split_whitespace()
        .map(|n| n.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    diff(&history)
}

fn diff(history: &[isize]) -> isize {
    let next = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    if next.iter().all(|n| *n == 0) {
        history[0]
    } else {
        history.last().unwrap() + diff(&next)
    }
}

fn main() {
    println!("{}", solve("./input/day9_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day9_1.sample.txt").unwrap(), 114);
    }
}
