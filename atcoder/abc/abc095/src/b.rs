use std::io;
use std::cmp;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let (n, mut x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut min = 10000000;
    for _ in 0..n {
        let m = read_one::<usize>();
        x -= m;
        min = cmp::min(m, min);
    }
    println!("{}", n + x / min);
}
