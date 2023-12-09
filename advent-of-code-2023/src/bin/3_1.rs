use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/3
/// Day 3: Gear Ratios
fn solve1(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut count = 0;

    let mut schematic = Vec::new();
    for line in lines {
        schematic.push(line?);
    }

    for (col, line) in schematic.iter().enumerate() {
        for (row, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                let parts = neighborhood_parts(col, row, &schematic);
                count += parts.into_iter().reduce(|acc, n| acc + n).unwrap_or(0);
            }
        }
    }

    Ok(count)
}
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut count = 0;
    let mut schematic = Vec::new();

    for (col, line) in lines.enumerate() {
        schematic.push(Vec::new());
        for c in line?.chars() {
            schematic[col].push(c);
        }
    }

    let parse = |cs: &[char]| cs.iter().collect::<String>().parse::<usize>().unwrap();

    for (col, line) in schematic.iter().enumerate() {
        let mut start = None;
        for (row, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                start.get_or_insert(row);
            } else if let Some(s) = start {
                if is_part(col, s, row, &schematic) {
                    count += parse(&schematic[col][s..row]);
                }
                start = None;
            }
        }

        if let Some(start) = start {
            if is_part(col, start, line.len(), &schematic) {
                count += parse(&schematic[col][start..]);
            }
        }
    }

    Ok(count)
}

fn neighborhood_parts(col: usize, row: usize, schematic: &[String]) -> Vec<usize> {
    let prev_col = col.saturating_sub(1);
    let next_col = (col + 1).min(schematic.len() - 1);
    let prev_row = row.saturating_sub(1);
    let next_row = (row + 1).min(schematic[col].len() - 1);

    let mut numbers = Vec::new();

    for column in &schematic[prev_col..=next_col] {
        let chunk = column[prev_row..=next_row]
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .count();
        for i in 0..chunk {
            let pos = column[prev_row..=next_row]
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
                .nth(i)
                .unwrap()
                .0
                + prev_row;
            let start = column[..=pos]
                .rfind(|c: char| !c.is_ascii_digit())
                .map(|p| p + 1)
                .unwrap_or(0);
            let end = column[pos..]
                .find(|c: char| !c.is_ascii_digit())
                .map(|end| end + pos)
                .unwrap_or(column.len());
            numbers.push(column[start..end].parse::<usize>().unwrap());
        }
    }

    numbers
}

fn is_part(col: usize, start: usize, end: usize, schematic: &[Vec<char>]) -> bool {
    let sym = |c: &char| !c.is_ascii_digit() && *c != '.';
    let check = |col: usize, start: usize, end: usize| schematic[col][start..end].iter().any(sym);

    let prev = start.saturating_sub(1);
    let next = (end + 1).min(schematic[col].len());

    check(col.saturating_sub(1), prev, next)
        || check((col + 1).min(schematic.len() - 1), prev, next)
        || sym(&schematic[col][prev])
        || sym(&schematic[col][next - 1])
}

fn main() {
    println!("{}", solve("./input/20.txt").unwrap());
    println!("{}", solve1("./input/20.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day3_1.sample.txt").unwrap(), 4361);
    }
}
