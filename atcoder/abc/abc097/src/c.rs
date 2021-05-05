use std::io;
use std::cmp;

fn main() {
    let cs = read_one::<String>();
    let cs_clone = cs.clone();
    let mut cs = cs.chars().zip(0..).collect::<Vec<(char, usize)>>();
    let k = read_one::<usize>();

    cs.sort();
    let mut vec = Vec::new();
    let mut cnt = 0;
    let mut prev = cs[0].0;
    let max = cs_clone.len();
    for c in cs {
        if cnt >= 5 { break; }
        if prev != c.0 { cnt += 1; prev = c.0; }
        for i in 0..5 {
            vec.push(&cs_clone[(c.1)..cmp::min(c.1 + i + 1, max)]);
        }
    }
    vec.sort();
    vec.dedup();
    println!("{}", vec[k-1]);
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
