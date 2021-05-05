use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;


fn main() {
    let (n, m, s, t) = {
        let i = read::<usize>();
        (i[0], i[1], i[2] - 1, i[3] - 1)
    };

    let mut vec_s = vec![Vec::new(); n];
    let mut vec_t = vec![Vec::new(); n];

    for _ in 0..m {
        let (u, v, a, b) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2], i[3])
        };

        vec_s[u].push(Edge { node: v, cost: a });
        vec_s[v].push(Edge { node: u, cost: a });
        vec_t[u].push(Edge { node: v, cost: b });
        vec_t[v].push(Edge { node: u, cost: b });
    }

    let dist_s = dijkstra(&vec_s, s);
    let dist_t = dijkstra(&vec_t, t);

    let mut ans = vec![::std::usize::MAX; n];
    for i in (0..n).rev() {
        ans[i] = dist_s[i] + dist_t[i];
        if i + 1 < n {
            ans[i] = ::std::cmp::min(ans[i], ans[i + 1]);
        }
    }
    for a in ans {
        println!("{}", 1_000_000_000_000_000 - a);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        match other.cost.cmp(&self.cost) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.pos.cmp(&other.pos)
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra(adj: &Vec<Vec<Edge>>, s: usize) -> Vec<usize> {
    let mut dist = (0..adj.len()).map(|_| ::std::usize::MAX)
        .collect::<Vec<usize>>();

    let mut q = BinaryHeap::new();
    dist[s] = 0;
    q.push( State { cost: 0, pos: s });

    while let Some( State { cost, pos }) = q.pop() {

        if cost > dist[pos] { continue; }

        for e in &adj[pos] {
            let s = State { cost: cost + e.cost, pos: e.node };
            if s.cost < dist[s.pos] {
                q.push(s);
                dist[s.pos] = s.cost;
            }
        }
    }

    dist
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
