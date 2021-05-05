use std::io;
use std::collections::VecDeque;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        let (u, v) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[u].push(v);
    }
    let (s, t) = {
        let i = read::<usize>();
        (i[0] - 1, i[1] - 1)
    };

    let mut que = VecDeque::new();
    let mut dist = vec![vec![::std::usize::MAX; 3]; n];
    que.push_back((s, 0));
    while let Some((u, v)) = que.pop_front() {
        if dist[u][v % 3] != ::std::usize::MAX { continue; }
        dist[u][v % 3] = v;

        for i in 0..g[u].len() {
            let vv = g[u][i];
            que.push_back((vv, v + 1));
        }
    }

    if dist[t][0] != ::std::usize::MAX { println!("{}", dist[t][0] / 3); }
    else { println!("-1"); }
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
