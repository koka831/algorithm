const MOD: usize = 1_000_000_000 + 7;

fn main() {
    let (n, k) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let mut cnt = 0;
    for i in k..=n + 1 {
        cnt += i * (2 * n + 1 - i) / 2 - i * (i - 1) / 2 + 1;
        cnt %= MOD;
    }

    println!("{}", cnt);
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
