use std::io;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let (h, n) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let pn = read::<usize>();
    let mut ans = 0;
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
