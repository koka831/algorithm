fn main() {
    let n = io::read_one::<usize>();
    let (a, b, c) = {
        let i = io::read::<usize>();
        (i[0], i[1], i[2])
    };


    let mut ans = std::usize::MAX;
    for na in 0..10_000 { for nb in 0..(10_000 - na) {
        let p = a * na + b * nb;
        if n >= p && (n - p) % c == 0 {
            ans = ans.min(na + nb + (n - p) / c);
        }
    }}

    println!("{}", ans);
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
