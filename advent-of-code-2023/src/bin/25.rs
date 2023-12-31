use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;

type Graph = HashMap<String, HashSet<String>>;

/// https://adventofcode.com/2023/day/25
/// Day 25: Snowverload
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let lines = std::io::read_to_string(file).unwrap();
    let lines = lines.trim().split('\n').collect::<Vec<_>>();
    let mut graph: Graph = HashMap::new();

    for line in lines {
        let (key, edges) = line
            .split_once(':')
            .map(|(k, e)| {
                (
                    k.trim().to_string(),
                    e.split_whitespace()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap();

        for edge in edges {
            graph.entry(key.clone()).or_default().insert(edge.clone());
            graph.entry(edge.clone()).or_default().insert(key.clone());
        }
    }

    for _ in 0..3 {
        let e = minimum_cut(&graph);
        graph.get_mut(&e.0).unwrap().remove(&e.1);
        graph.get_mut(&e.1).unwrap().remove(&e.0);
    }

    let s = graph.keys().next().unwrap();
    let part = bfs(s.clone(), &graph);
    Ok(part * (graph.len() - part))
}

// find a path (from, to)
fn minimum_cut(graph: &Graph) -> (String, String) {
    let mut paths: HashMap<(&String, &String), usize> = HashMap::new();
    for s in graph.keys() {
        let mut nodes = VecDeque::new();
        let mut visited = HashSet::new();

        nodes.push_back(s);
        visited.insert(s);

        while let Some(u) = nodes.pop_front() {
            for v in graph[u].iter() {
                if !visited.contains(v) {
                    nodes.push_back(v);
                    visited.insert(v);

                    let e = if v < u { (v, u) } else { (u, v) };
                    *paths.entry(e).or_default() += 1;
                }
            }
        }
    }

    paths
        .iter()
        .max_by_key(|&(_, v)| v)
        .map(|(v, _)| (v.0.to_owned(), v.1.to_owned()))
        .unwrap()
}

fn bfs(v: String, g: &Graph) -> usize {
    let mut cnt = 0;
    let mut visited = HashSet::new();
    let mut que = VecDeque::new();
    que.push_back(&v);
    while let Some(v) = que.pop_front() {
        if visited.contains(&v) { continue; }
        visited.insert(v);
        cnt += 1;

        for u in g.get(v).unwrap() {
            if u != v {
                que.push_back(u);
            }

        }
    }

    cnt
}

fn main() {
    println!("{}", solve("./input/day25.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day25.sample.txt").unwrap(), 54);
    }
}
