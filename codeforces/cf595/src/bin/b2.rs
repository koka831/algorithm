use std::io;

fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let n = read_one::<usize>();
    let pn = read::<usize>()
        .iter()
        .map(|i| i - 1)
        .collect::<Vec<usize>>();
    let mut ans = vec![0; n];
    for p in 0..n {
        if ans[p] != 0 { continue; }
        let mut vec = Vec::new();
        let mut next = pn[p];
        loop {
            vec.push(next);
            if next == p { break; }
            next = pn[next];
        }

        for &i in vec.iter() {
            ans[i] = vec.len();
        }
    }
    for a in ans {
        print!("{} ", a);
    }
    println!();
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
