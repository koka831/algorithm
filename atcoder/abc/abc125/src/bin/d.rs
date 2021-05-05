use std::cmp;
use std::io;

fn main() {
    let n = read_one::<usize>();
    let an = read::<isize>();
    let mut dp = vec![vec![0isize; 2]; n + 1];
    dp[0][1] = -1_000_000_000;
    for i in 0..n {
        dp[i + 1][0] = cmp::max(dp[i][0] + an[i], dp[i][1] - an[i]);
        dp[i + 1][1] = cmp::max(dp[i][0] - an[i], dp[i][1] + an[i]);
    }

    println!("{}", dp[n][0]);
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
