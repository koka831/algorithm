use std::fs::File;
use std::io::{BufRead, BufReader};

/// https://adventofcode.com/2023/day/2
/// Day 2: Cube Conundrum
fn solve(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(&file).lines();
    let mut count = 0;

    for line in lines {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<_>>();

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for v in line[2..].chunks(2) {
            let (num, color) = (v[0].parse::<usize>().unwrap(), v[1].replace([';', ','], ""));

            match color.as_str() {
                "red" => red = red.max(num),
                "green" => green = green.max(num),
                "blue" => blue = blue.max(num),
                _ => unreachable!(),
            };
        }

        count += red * green * blue;
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
        assert_eq!(solve("./input/day2_1.sample.txt"), 2286);
    }
}
