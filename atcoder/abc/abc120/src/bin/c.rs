use std::io;
use std::cmp;


fn main() {
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let red = s.iter().filter(|&c| *c == '0').count();
    let blue = s.iter().filter(|&c| *c == '1').count();

    println!("{}", cmp::min(red, blue) * 2);
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
