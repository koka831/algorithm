use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/18
/// Day 18: Lavaduct Lagoon
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();

    let mut position = (0, 0);
    let mut edges = vec![position];

    while let Some(Ok(line)) = lines.next() {
        let mut l = line.split_whitespace();
        let direction = l.next().unwrap();
        let distance = l.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "U" => position.1 -= distance,
            "D" => position.1 += distance,
            "L" => position.0 -= distance,
            "R" => position.0 += distance,
            _ => panic!("Invalid direction"),
        }
        edges.push(position);
    }
    Ok(fill_interior(&edges))
}

fn fill_interior(vertices: &Vec<(i32, i32)>) -> usize {
    let mut area = 0.0;

    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };

        area += (x1 * y2) as f64 - (x2 * y1) as f64;
    }

    let area = area.abs() as usize / 2;
    vertices.len() / 2 + area + 1
}

fn main() {
    println!("{}", solve("./input/day18.sample.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day18.sample.txt").unwrap(), 62);
    }
}
