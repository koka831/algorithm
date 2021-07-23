fn main() {
    let (n, k) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };
    let base10 = rebase(n, 8, 9);
    println!("{}", base9);
}

fn rebase(n: usize, from: usize, to: usize) -> usize {
    let mut n = n;
    let mut result = 0;

    let mut i = 1;
    while n > 0 {
        let d = n % to;
        n /= to;
        result += d * (to.pow(i as u32));

        i *= to;
    }

    result
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
