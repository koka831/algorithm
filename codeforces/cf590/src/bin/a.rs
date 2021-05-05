use std::io;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let ans = (an.iter().sum::<usize>() as f64 / n as f64).ceil();
    println!("{}", ans);
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
