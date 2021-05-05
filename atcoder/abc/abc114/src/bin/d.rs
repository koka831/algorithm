use std::io;
use std::collections::HashMap;


fn main() {
    let x = read_one::<u64>();
    let mut hm = HashMap::new();
    for i in 1..x + 1 {
        let mut m = i;
        while let Some(f) = divide(m) {
            m /= f;
            *hm.entry(f).or_insert(0) += 1;
        }
        *hm.entry(m).or_insert(0) += 1;
    }

    *hm.entry(1).or_insert(0) = 1;
    // 75 = 3, 5, 5
    let mut c2 = 0;
    let mut c4 = 0;
    let mut c14 = 0;
    let mut c24 = 0;
    let mut c74 = 0;
    for v in hm.values() {
        if v >= &74 { c74 += 1; }
        if v >= &24 { c24 += 1; }
        if v >= &14 { c14 += 1; }
        if v >= &4 { c4 += 1; }
        if v >= &2 { c2 += 1; }
    }

    let mut ans = c74;
    if c4 >= 2 && c2 >= 3 {
        ans += ncr(c4, 2) * (c2 - 2);
    }

    if c14 >= 1 && c4 >= 2 {
        ans += c14 * (c4 - 1);
    }

    if c24 >= 1 && c2 >= 2 {
        ans += c24 * (c2 - 1);
    }

    println!("{}", ans);
}

fn divide(n: u64) -> Option<u64> {
    for i in 2..(n as f64).sqrt().ceil() as u64 + 2 {
        if n % i == 0 {
            return Some(i);
        }
    } 
    return None;
}

fn ncr(n: u64, r: u64) -> u64 {
    if r > n { return 0; }
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ((ncr(n, r - 1) * (n - r + 1) / r)),
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
