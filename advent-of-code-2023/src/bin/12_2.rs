use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2023/day/12
/// Day 12: Hot Springs
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = BufReader::new(&file).lines();

    let mut count = 0;
    let mut hm = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let (log, contiguous) = {
            let line = line.split_whitespace().collect::<Vec<_>>();
            let log = String::from(line[0]) + "?";
            let mut log = log.repeat(5).chars().collect::<Vec<_>>();
            // remove extra `?`
            log.pop();

            let contiguous = line[1]
                .split(',')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            (log, contiguous)
        };

        let c = candidates(&log, &contiguous, &mut hm);
        count += c;
    }

    Ok(count)
}

fn k(a: &[char], b: &[usize]) -> u64 {
    let mut hasher = DefaultHasher::new();
    a.hash(&mut hasher);
    b.hash(&mut hasher);
    hasher.finish()
}

fn candidates(log: &[char], contiguous: &[usize], cache: &mut HashMap<u64, usize>) -> usize {
    if let Some(value) = cache.get(&k(log, contiguous)) {
        return *value;
    }

    match (log.is_empty(), contiguous.is_empty()) {
        (true, true) => return 1,
        (true, false) => return 0,
        (false, true) => {
            return if log.contains(&'#') { 0 } else { 1 };
        }
        _ => {}
    }

    let res = match log[0] {
        '.' => candidates(&log[1..], contiguous, cache),
        '?' => {
            let mut a = Vec::from(log);
            a[0] = '#';

            candidates(&a, contiguous, cache) + candidates(&log[1..], contiguous, cache)
        }
        '#' => {
            let n = contiguous[0];
            if log.len() >= n
                && log[..n].iter().all(|c| *c != '.')
                // terminated or separated by . or ?
                && (log.len() == n || log[n] != '#')
            {
                candidates(&log[(n + 1).min(log.len())..], &contiguous[1..], cache)
            } else {
                0
            }
        }
        _ => unreachable!(),
    };

    cache.insert(k(log, contiguous), res);
    res
}

fn main() {
    println!("{}", solve("./input/day12_1.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day12_1.sample.txt").unwrap(), 525152);
    }
}
