use std::io;


fn main() {
    let s = read_one::<String>().chars()
        .collect::<Vec<_>>();
    let t = read_one::<String>().chars()
        .collect::<Vec<_>>();

    let ans = s.iter().zip(&t)
        .filter(|(a, b)| a != b)
        .count();
    println!("{}", ans);
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
