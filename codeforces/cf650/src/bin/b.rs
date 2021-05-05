use std::io;

fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    // need to move
    let mut cnt_even = 0;
    let mut cnt_odd = 0;
    for i in 0..n {
        if an[i] % 2 != i % 2 {
            if i % 2 == 0 { cnt_even += 1; }
            else { cnt_odd += 1; }
        }
    }

    if cnt_odd != cnt_even { println!("-1"); }
    else { println!("{}", cnt_even); }
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
