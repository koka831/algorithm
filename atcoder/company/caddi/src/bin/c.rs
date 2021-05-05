use std::io;
use std::collections::HashMap;


fn main() {
    let (n, p) = {
        let i = read::<u64>();
        (i[0], i[1])
    };

    let mut hm = HashMap::new();
    let mut x = p;
    while let Some(f) = divide(x) {
        x /= f;
        *hm.entry(f).or_insert(0) += 1;
    }
    *hm.entry(x).or_insert(0) += 1;

    let mut ans: u64 = 1;

    for (k, v) in hm.iter() {
        if v >= &n {
            ans *= k.pow((v / n) as u32);
        }
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
