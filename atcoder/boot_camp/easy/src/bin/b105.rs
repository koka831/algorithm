use std::io;

fn main() {
    let n = read_one::<usize>();
    let ans = (1..=n)
        .filter(|&x| x % 2 != 0)
        .filter(|&x| divisors(x) == 8)
        .count();
    println!("{}", ans);
}

fn divisors(n: usize) -> usize{
    let mut ret = Vec::new();
    for i in 1..=n {
        if n % i == 0 { ret.push(i); }
    }
    ret.len()
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
