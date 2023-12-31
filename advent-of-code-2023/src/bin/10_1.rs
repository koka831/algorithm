use std::fs::File;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
    // north and south
    NS,
    // east and west
    EW,
    // north and east
    NE,
    // north and west
    NW,
    // south and west
    SW,
    // south and east
    SE,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        use Direction::*;

        match value {
            "|" => NS,
            "-" => EW,
            "L" => NE,
            "J" => NW,
            "7" => SW,
            "F" => SE,
            _ => unreachable!("unknown direction {}", value),
        }
    }
}

/// https://adventofcode.com/2023/day/10
/// Day 10: Pipe Maze
fn solve(path: &str) -> io::Result<isize> {
    let file = File::open(path)?;
    let map = std::io::read_to_string(file)?
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .position(|col| col.contains(&'S'))
        .map(|i| (i, map[i].iter().position(|row| row == &'S').unwrap()))
        .unwrap();

    let mut deq = Vec::new();
    let mut max_dist = 0;
    deq.push((start, max_dist));
    while let Some(((col, row), dist)) = deq.pop() {}

    dbg!(start);

    Ok(0)
}

fn main() {
    println!("{}", solve("./input/day10_2.sample.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        // assert_eq!(solve("./input/day10_1.sample.txt").unwrap(), 4);
        // assert_eq!(solve("./input/day10_2.sample.txt").unwrap(), 8);
    }
}
