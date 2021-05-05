use std::io;
use std::collections::HashMap;

fn main() {
    let n = read_one::<usize>();
    let mut hm = HashMap::new();
    for _ in 0..n {
        let s = read_one::<String>();
        *hm.entry(s).or_insert(0) += 1;
    }

    println!("AC x {}", hm.get("AC".into()).unwrap_or(&0));
    println!("WA x {}", hm.get("WA".into()).unwrap_or(&0));
    println!("TLE x {}", hm.get("TLE".into()).unwrap_or(&0));
    println!("RE x {}", hm.get("RE".into()).unwrap_or(&0));
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
