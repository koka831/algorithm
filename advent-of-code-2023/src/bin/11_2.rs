#![allow(clippy::needless_range_loop)]
use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/11
/// Day 11: Cosmic Expansion
fn solve(path: &str, expansion: usize) -> io::Result<usize> {
    let file = File::open(path)?;
    let map = std::io::read_to_string(file)?
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row = map.len();
    let col = map[0].len();
    let space = '.';
    let galaxy = '#';

    let mut acc_y = vec![0; row];
    for i in 0..row {
        if map[i].iter().all(|x| *x == space) {
            acc_y[i] += 1;
        }

        if i > 0 {
            acc_y[i] += acc_y[i - 1];
        }
    }

    let mut acc_x = vec![0; col];
    for i in 0..col {
        if (0..row).all(|j| map[j][i] == space) {
            acc_x[i] += 1;
        }

        if i > 0 {
            acc_x[i] += acc_x[i - 1];
        }
    }

    let dist = |(x, y): (usize, usize)| -> usize {
        let mut distance = 0;
        for r in 0..row {
            for c in 0..col {
                if map[r][c] == galaxy {
                    let d = ((y + acc_y[y] * expansion) as isize
                        - (r + acc_y[r] * expansion) as isize)
                        .abs()
                        + ((x + acc_x[x] * expansion) as isize
                            - (c + acc_x[c] * expansion) as isize)
                            .abs();

                    distance += d;
                }
            }
        }

        distance as usize
    };

    let mut score = 0;
    for r in 0..row {
        for c in 0..col {
            if map[r][c] == galaxy {
                score += dist((c, r));
            }
        }
    }

    Ok(score / 2)
}

fn main() {
    println!("{}", solve("./input/day11_1.txt", 1_000_000 - 1).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day11_1.sample.txt", 9).unwrap(), 1030);
        assert_eq!(solve("./input/day11_1.sample.txt", 99).unwrap(), 8410);
    }
}
