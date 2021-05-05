use std::io;
use std::collections::HashMap;


fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let t = read_one::<String>().chars().collect::<Vec<char>>();

    let mut flg = true;
    let mut s_t = HashMap::new();
    let mut t_s = HashMap::new();
    for i in 0..s.len() {
        let j = s_t.entry(s[i]).or_insert(i);
        let k = t_s.entry(t[i]).or_insert(i);
        if t[*j] != t[i] { flg = false; }
        if s[*k] != s[i] { flg = false; }
    }

    if flg { println!("Yes"); }
    else { println!("No"); }
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
