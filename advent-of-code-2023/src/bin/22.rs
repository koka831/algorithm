use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}
impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    start: Point,
    end: Point,
    supports: HashSet<Point>,
}

impl Brick {
    fn new(start: Point, end: Point) -> Self {
        let supports = HashSet::new();
        Brick {
            start,
            end,
            supports,
        }
    }
}

/// https://adventofcode.com/2023/day/22
/// Day 22: Sand Slabs
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();

    let mut bricks = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let parts: Vec<usize> = line
            .split(['~', ','].as_ref())
            .map(|s| s.parse().unwrap())
            .collect();
        let start = Point::new(parts[0], parts[1], parts[2]);
        let end = Point::new(parts[3], parts[4], parts[5]);
        bricks.push(Brick::new(start, end));
    }

    bricks.sort_unstable_by_key(|b| b.start.z);

    let mut index = vec![vec![std::usize::MAX; 10]; 10];
    let mut heights = vec![vec![0; 10]; 10];
    let mut fall = vec![Vec::new(); bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        let height = brick.end.z - brick.start.z + 1;
        let top = *heights.iter().flatten().max().unwrap();
        let mut prev = std::usize::MAX;

        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                if heights[y][x] == top {
                    let idx = index[y][x];
                    if idx != prev {
                        fall[idx].push(i);
                        prev = idx;
                    }
                }

                heights[y][x] = top + height;
                index[y][x] = i;
            }
        }
    }

    let mut safe = vec![true; fall.len()];
    dbg!(&fall);
    for brick in fall.iter() {
        if brick.len() == 1 {
            safe[brick[0]] = false;
        }
    }

    let safe = safe.iter().filter(|b| **b).count();
    dbg!(safe);
    let safe = fall.iter().filter(|b| b.len() != 1).count();

    Ok(safe)
}

fn main() {
    println!("{}", solve("./input/day22.sample.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day22.sample.txt").unwrap(), 5);
    }
}
