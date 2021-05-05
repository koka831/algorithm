use std::io;
use std::collections::HashMap;


fn main() {
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut hm = HashMap::new();
    for c in s {
        *hm.entry(c).or_insert(0) += 1;
    }

    let mut cnt: usize = 0;
    let mut odd = 0;

    for (_, val) in hm { cnt += val / 2 * 2; odd += val % 2; }

    if odd > 0 {
        cnt += 1;
        odd -= 1;
    }

    let score = cnt.pow(2) + odd;

    println!("{}", score);
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
