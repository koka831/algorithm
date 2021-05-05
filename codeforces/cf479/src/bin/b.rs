use std::io;
use std::collections::HashMap;


fn main() {
    let n = read_one::<usize>();
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut hm = HashMap::new();

    for i in 0..n - 1 {
        *hm.entry(s[i..i + 2].iter().collect::<String>()).or_insert(0) += 1;
    }

    let mut v: Vec<_> = hm.iter().collect();
    v.sort_by_key(|&(_, v)| v);
    v.reverse();
    println!("{}", v[0].0);
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
