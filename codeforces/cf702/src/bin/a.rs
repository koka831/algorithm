use std::io;
use std::cmp;

fn main() {
    let t = read_one::<usize>();
    for _ in 0..t {
        let _n = read_one::<usize>();
        let an = read::<usize>();

        while let Some([a, b]) = an.windows(2).next() {
            let min = cmp::min(a, b);
            let max = cmp::max(a, b);
        }
    }
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
