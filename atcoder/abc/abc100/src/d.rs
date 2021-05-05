use std::io;
use std::cmp;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut x = Vec::new();
    let mut dp = Vec::new();
    for j in 0..n {
        x.push(read::<u64>());
        dp.push(Vec::new());
        for _ in 0..m {
            dp[j].push(0);
        }
    }

    for j in 0..n { for k in 0..m {
        if k < m - 1 {
            dp[j + 1][k] = cmp::max(dp[j][k - 1], dp[j][k]);
        } else {
            dp[j + 1][k] = dp[j][k];
        }
    }}
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
