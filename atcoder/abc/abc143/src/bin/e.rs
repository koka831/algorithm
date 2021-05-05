use std::io;


fn main() {
    let (n, m, l) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut dist = vec![vec![0; n]; n];
    for _ in 0..m {
        let (a, b, c) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1, i[2])
        };
        dist[a][b] = c;
        dist[b][a] = c;
    }

    let mut dp = vec![vec![100_000_000_000; n]; n];
    for i in 0..n { for j in 0..n {
        if dist[i][j] > 0 { dp[i][j] = dist[i][j]; }
    }}

    for k in 0..n { for i in 0..n { for j in 0..n {
        if dp[i][k] + dp[k][j] < dp[i][j] {
            dp[i][j] = dp[i][k] + dp[k][j];
        }
    }}}

    for i in 0..n { for j in 0..n {
        dp[i][j] = if dp[i][j] <= l { 1 }
        else { 100_000_000_000 };
    }}

    for k in 0..n { for i in 0..n { for j in 0..n {
        if dp[i][k] + dp[k][j] < dp[i][j] {
            dp[i][j] = dp[i][k] + dp[k][j];
        }
    }}}

    let q = read_one::<usize>();
    for _ in 0..q {
        let (s, t) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        let ans: isize = if dp[s][t] > 100_000_000 { -1 }
        else { (dp[s][t] - 1) as isize };
        println!("{}", ans);
    }
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
