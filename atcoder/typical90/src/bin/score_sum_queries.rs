fn main() {
    let n = io::read_one::<usize>();

    let mut an = vec![0; n + 1];
    let mut bn = vec![0; n + 1];

    for i in 0..n {
        let (c, p) = {
            let i = io::read::<usize>();
            (i[0], i[1])
        };

        if c == 1 {
            an[i + 1] = an[i] + p;
            bn[i + 1] = bn[i];
        } else {
            an[i + 1] = an[i];
            bn[i + 1] = bn[i] + p;
        }
    }

    let q = io::read_one::<usize>();

    for _ in 0..q {
        let (l, r) = {
            let i = io::read::<usize>();
            (i[0], i[1])
        };

        println!("{} {}", an[r] - an[l - 1], bn[r] - bn[l - 1]);
    }
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
