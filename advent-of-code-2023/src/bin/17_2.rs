use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/17
/// Day 17: Clumsy Crucible
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = std::io::read_to_string(file).unwrap();
    let grid = lines
        .trim()
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| (c as u8 - b'0') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let min = search(&grid, (0, 0));
    Ok(min[grid.len() - 1][grid[0].len() - 1] - grid[0][0])
}

struct E {
    x: usize,
    y: usize,
    cost: usize,
    count: usize,
    direction: isize,
}
impl PartialEq for E {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for E {}
/// necessary for BinaryHeap priority
impl Ord for E {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for E {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search(grid: &Vec<Vec<usize>>, (sx, sy): (usize, usize)) -> Vec<Vec<usize>> {
    let (xlen, ylen) = (grid[0].len(), grid.len());
    let mut costs = vec![vec![std::usize::MAX; xlen]; ylen];
    costs[sy][sx] = grid[sy][sx];

    let mut q = BinaryHeap::from([E {
        cost: grid[sy][sx],
        direction: 2,
        count: 4,
        x: sx,
        y: sy,
    }]);
    let mut visited = HashSet::new();

    while let Some(E {
        cost,
        direction,
        count,
        x,
        y,
    }) = q.pop()
    {
        if visited.contains(&(x, y, direction, count)) {
            continue;
        }
        visited.insert((x, y, direction, count));

        // direction: (1, 2, -1, -2) = (up, right, down, left)
        let adjucents = [
            y.checked_sub(1).map(|y| (x, y, 1)),
            x.checked_sub(1).map(|x| (x, y, -2)),
            (x < xlen - 1).then_some((x + 1, y, 2)),
            (y < ylen - 1).then_some((x, y + 1, -1)),
        ];
        for &(ux, uy, dir) in adjucents
            .iter()
            .flatten()
            .filter(|(_, _, d)| d + direction != 0)
            .filter(|(_, _, d)| (*d == direction && count < 10) || (*d != direction && count >= 4))
        {
            let c = cost + grid[uy][ux];
            let count = if dir == direction { count + 1 } else { 1 };

            if c < costs[uy][ux] {
                if (ux, uy) == (xlen - 1, ylen - 1) {
                    if count >= 4 {
                        costs[uy][ux] = c;
                    }
                } else {
                    costs[uy][ux] = c;
                }
            }
            q.push(E {
                cost: c,
                direction: dir,
                count,
                x: ux,
                y: uy,
            });
        }
    }

    for l in &costs {
        for j in l {
            print!("{j:3} ");
        }
        println!();
    }

    costs
}

fn main() {
    println!("{}", solve("./input/day17.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day17.sample.txt").unwrap(), 102);
    }
}
