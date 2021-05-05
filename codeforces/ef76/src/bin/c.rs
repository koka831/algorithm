use std::io;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let _ = read_one::<usize>();
    let an = read::<u64>();

    let mut hm: HashMap<u64, Vec<u64>> = HashMap::new();
    for (i, a) in an.iter().enumerate() {
        hm.entry(*a).or_insert(Vec::new()).push(i as u64);
    }

    let mut ans: u64 = 10000000000;
    for v in hm.values() {
        if v.len() == 1 { continue; }
        let mut prev = v[0];
        for i in 1..v.len() {
            let cur = v[i];
            ans = cmp::min(ans, cur - prev + 1);
            prev = cur;
        }
    }

    if ans == 10000000000 { println!("-1"); }
    else { println!("{}", ans); }
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
