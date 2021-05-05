use std::io;

const MOD: usize = 1_000_000_007;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = vec![false; n];
    for _ in 0..m { an[read_one::<usize>() - 1] = true; }

    if n == 1 {
        let ans = if an[0] { 0 } else { 1 };
        println!("{}", ans);
        return;
    }

    let mut dp = vec![0; n];
    dp[0] = if an[0] { 0 } else { 1 };
    dp[1] = if an[1] { 0 } else { 1 + dp[0] };
    for i in 2..n {
        if an[i] { continue; }
        dp[i] = (dp[i - 1] + dp[i - 2]) % MOD;
    }
    println!("{}", dp[n - 1]);
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
