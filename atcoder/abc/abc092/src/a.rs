use std::io;
use std::cmp;

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let a = read_one::<isize>();
    let b = read_one::<isize>();
    let c = read_one::<isize>();
    let d = read_one::<isize>();

    println!("{}", cmp::min(a, b) + cmp::min(c, d));
}
