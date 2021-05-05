use std::io;
use std::cmp;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let n = read_one::<u64>();
    let mut ans: u64 = std::u64::MAX;
    for i in 0..1 << 20 {
        let mut sum: u64 = 0;
        for j in 0..21 {
            if i & (1 << j) != 0 {
                sum += 3u64.pow(j);
            }
        }
        if sum >= n {
            ans = cmp::min(ans, sum);
        }
    }
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
