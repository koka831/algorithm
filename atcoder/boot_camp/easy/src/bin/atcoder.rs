use std::io;
use std::cmp;

fn main() {
    let s = read_one::<String>().chars()
        .collect::<Vec<_>>();

    let mut cnt = 0;
    let mut len = 0;
    for c in s.iter() {
        if ['A', 'C', 'G', 'T'].contains(c) {
            len += 1;
        } else {
            len = 0;
        }
        cnt = cmp::max(cnt, len);
    }

    println!("{}", cnt);
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
