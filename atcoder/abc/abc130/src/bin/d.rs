use std::io;

fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let mut acc = vec![0; n];
    for i in 1..n {
        acc[i] = acc[i - 1] + an[i - 1];
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
