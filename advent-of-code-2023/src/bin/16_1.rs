use std::collections::HashSet;
use std::fs::File;
use std::io;

/// https://adventofcode.com/2023/day/16
/// Day 16: The Floor Will Be Lava
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = std::io::read_to_string(file).unwrap();
    let grid = lines
        .trim()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut queue: Vec<(isize, isize, isize, isize)> = Vec::new();
    let mut energized = HashSet::new();
    // x, y, dx, dy
    queue.push((0, 0, 1, 0));

    while let Some((mut x, mut y, mut dx, mut dy)) = queue.pop() {
        'inner: while x >= 0 && (x as usize) < grid[0].len() && y >= 0 && (y as usize) < grid.len()
        {
            if energized.contains(&(x, y, dx, dy)) { break 'inner; }

            energized.insert((x, y, dx, dy));
            match grid[y as usize][x as usize] {
                '/' => {
                    dx = -dx;
                    dy = -dy;
                    std::mem::swap(&mut dx, &mut dy);
                }
                '\\' => {
                    std::mem::swap(&mut dx, &mut dy);
                }
                '|' if dx != 0 => {
                    queue.push((x, y + 1, 0, 1));
                    queue.push((x, y - 1, 0, -1));
                    continue;
                }
                '-' if dy != 0 => {
                    queue.push((x + 1, y, 1, 0));
                    queue.push((x - 1, y, -1, 0));
                    continue;
                }
                _ => {}
            }
            x += dx;
            y += dy;
            assert!(dx != dy);
            queue.push((x, y, dx, dy));
        }
    }

    // Visualizer:
    // let mut g = vec![vec!['.'; grid[0].len()]; grid.len()];
    // for line in &energized {
    //     g[line.1 as usize][line.0 as usize] = '#';
    // }
    // for line in &g {
    //     println!("{}", String::from_iter(line));
    // }
    let energized = energized.iter().map(|e| (e.0, e.1)).collect::<HashSet<_>>();
    Ok(energized.len())
}

fn main() {
    println!("{}", solve("./input/day16.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day16.sample.txt").unwrap(), 46);
    }
}
