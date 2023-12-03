use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2023/day/3
/// Day 3: Gear Ratios
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut schematic = Vec::new();

    for line in lines {
        schematic.push(line?);
    }

    let mut count = 0;

    for (col, line) in schematic.iter().enumerate() {
        for (row, c) in line.chars().enumerate() {
            if c == '*' {
                let parts = neighborhood_parts(col, row, &schematic);
                if parts.len() >= 2 {
                    count += parts.into_iter().reduce(|acc, n| acc * n).unwrap_or(0);
                }
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
            .filter(|s| !s.is_empty());
        for i in 0..chunk.count() {
            // find n-th index
            let pos = column[prev_row..=next_row]
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
                .map(|(i, _)| i)
                .nth(i)
                .unwrap()
                + prev_row;
            let start = column[..=pos]
                .rfind(|c: char| !c.is_ascii_digit())
                .map(|p| p + 1)
                .unwrap_or(0);
            let end = column[pos..]
                .find(|c: char| !c.is_ascii_digit())
                .map(|p| p + pos)
                .unwrap_or(column.len());
            numbers.push(column[start..end].parse::<usize>().unwrap());
        }
    }

    numbers
}

fn main() {
    println!("{}", solve("./input/day3_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day3_1.sample.txt").unwrap(), 467835);
    }
}
