use std::io;
use std::iter::FromIterator;
use std::collections::BTreeSet;


fn main() {
    let mut s = read_one::<String>().chars().collect::<Vec<char>>();

    let q = read_one::<usize>();

    for _ in 0..q {
        let i = read::<String>();
        let t = i[0].parse::<usize>().unwrap();
        let a = i[1].parse::<usize>().unwrap() - 1;
        let b = i[2].clone();
        if t == 1 {
            s[a] = b.chars().collect::<Vec<char>>()[0];
        } else {
            let hs = BTreeSet::from_iter(&s[a..b.parse::<usize>().unwrap()]);
            println!("{}", hs.len());
        }
    }
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
