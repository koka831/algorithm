fn main() {
    let (h, n) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let i = io::read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }

    let mut dp = vec![vec![1_000_000_000_000; h + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n { for j in 0..=h {
        let k = h.min(j + ab[i].0);

        dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
        dp[i + 1][k] = dp[i + 1][k].min(dp[i + 1][j] + ab[i].1);
    }}

    println!("{}", dp[n][h]);
}

#[allow(unused)]
mod io {
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
