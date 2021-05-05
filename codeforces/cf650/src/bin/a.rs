use std::io;

fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        if i > 1 && i % 2 == 0 { continue; }
        print!("{}", s[i]);
    }
    println!();
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
