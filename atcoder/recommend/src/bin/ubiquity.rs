const modulo: isize = 1_000_000_007;

fn main() {
    let n = io::read_one::<isize>();

    let ans = mod_pow(10, n) + mod_pow(8, n) - mod_pow(9, n) - mod_pow(9, n);
    println!("{}", (ans % modulo + modulo) % modulo);
}

fn mod_pow(i: isize, p: isize) -> isize {
    let mut res = 1;
    for _ in 0..p {
        res = res * i % modulo;
    }

    res
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
