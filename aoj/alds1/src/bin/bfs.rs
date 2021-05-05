use std::io;
use std::cmp;
use std::collections::VecDeque;

fn main() {
    let n = read_one::<usize>();
    let mut g = Vec::new();
    for _ in 0..n {
        let vs = read::<usize>()[2..].iter()
            .map(|v| v - 1)
            .collect::<Vec<_>>();
        g.push(vs);
    }

    let mut cost = vec![!0; n];
    let mut arrived = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0));

    while let Some((v, p)) = q.pop_front() {
        arrived[v] = true;
        cost[v] = cmp::min(cost[v], 1usize.wrapping_add(cost[p]));

        for &u in &g[v] {
            if u == p || arrived[u] { continue; }
            q.push_back((u, v));
        }
    }

    for (i, &c) in cost.iter().enumerate() {
        print!("{} ", i + 1);
        println!("{}", if arrived[i] { c as isize } else { -1 });
    }
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
