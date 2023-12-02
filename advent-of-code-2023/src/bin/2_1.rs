use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// https://adventofcode.com/2023/day/2
/// Day 2: Cube Conundrum
fn solve(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(&file).lines();
    let mut count = 0;

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<_>>();

        let mut valid = true;
        for v in line[2..].chunks(2) {
            let (num, color) = (v[0].parse::<usize>().unwrap(), v[1].replace([';', ','], ""));

            valid &= match color.as_str() {
                "red" => num <= 12,
                "green" => num <= 13,
                "blue" => num <= 14,
                _ => unreachable!(),
            };
        }

        if valid {
            count += i + 1;
        }
    }

    count
}

fn main() {
    println!("{}", solve("./input/day2_1.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day2_1.sample.txt"), 8);
    }
}
