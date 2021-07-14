fn main() {
    let (n, p, q) = {
        let i = io::read::<usize>();
        (i[0], i[1], i[2])
    };
    let an = io::read::<usize>();

    let mut ans = 0;
    for i in 0..n { for j in (i + 1)..n { for k in (j + 1)..n { for l in (k + 1)..n { for m in (l + 1)..n {
        if (((an[i] * an[j]) % p * an[k] % p) * an[l] % p) * an[m] % p == q { ans += 1; }
    }}}}}

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
