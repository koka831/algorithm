const MODULO: usize = 998_244_353;

fn main() {
    let n = io::read_one::<usize>();
    let an = io::read::<usize>();

    let mut dp = vec![vec![0; 10]; n + 1];
    dp[0][an[0]] = 1;

    for i in 0..n - 1 {
        for j in 0..10 {
            dp[i + 1][(j + an[i + 1]) % 10] += dp[i][j];
            dp[i + 1][(j * an[i + 1]) % 10] += dp[i][j];

            dp[i + 1][(j + an[i + 1]) % 10] %= MODULO;
            dp[i + 1][(j * an[i + 1]) % 10] %= MODULO;
        }
    }

    dbg!(&dp);

    for i in 0..10 {
        println!("{}", dp[n][i]);
    }
}

pub mod io {
    use std::io;
    pub fn read<T>() -> Vec<T>
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
    pub fn read_one<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
