use std::io;
use std::collections::VecDeque;
use std::cmp;

fn main() {
    let n = read_one::<usize>();
    let mut g = Vec::new();
    for _ in 0..n {
        let vs = read::<usize>()[2..].iter()
            .map(|v| v - 1)
            .collect::<Vec<_>>();
        g.push(vs);
    }

    println!("{:?}", g);

    let mut begin = vec![std::usize::MAX; n];
    let mut finish = vec![0; n];
    let mut cnt = 0;
    let mut q = VecDeque::new();
    q.push_front((0, 0));

    while let Some((v, p)) = q.pop_front() {
        cnt += 1;
        begin[v] = cmp::min(begin[v], cnt);
        for &u in g[v].iter() {
            if u == p { continue; }
            q.push_back((u, v));
        }
    }

    for i in 0..n {
        finish[i] = n * 2 + 1 - begin[i];
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, begin[i], finish[i]);
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
