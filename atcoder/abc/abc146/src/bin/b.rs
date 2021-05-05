use std::io;


fn main() {
    let n = read_one::<u8>();
    let ss = read_one::<String>().chars()
        .map(|c| (n + (c as u8) - b'A') % 26)
        .map(|i| (i + b'A') as char)
        .collect::<String>();
    println!("{}", ss);
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
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
