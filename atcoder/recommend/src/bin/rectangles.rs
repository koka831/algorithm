fn main() {
    let n = io::read_one::<usize>();
    let mut xy = Vec::new();
    for _ in 0..n {
        let (x, y) = {
            let i = io::read::<usize>();
            (i[0], i[1])
        };

        xy.push((x, y));
    }

    xy.sort();

    let mut cnt = 0;
    for &a in &xy {
        for &b in &xy {
            if a.0 >= b.0 || a.1 >= b.1 {
                continue;
            }

            if xy.binary_search(&(a.0, b.1)).is_ok() && xy.binary_search(&(b.0, a.1)).is_ok() {
                cnt += 1;
            }
        }
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
