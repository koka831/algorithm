use std::io;
use std::collections::HashMap;


fn main() {
    let _ = read_one::<usize>();
    let an = read::<usize>();
    let mut hm = HashMap::new();
    for a in an.iter() {
        let xn = prime_factor(*a);
        println!("{:?}", xn);
        for x in xn {
            *hm.entry(x).or_insert(0) += 1;
        }
    }
    let mut ans = 1;
    for x in hm.values() {
        if *x >= an.len() { ans *= x / an.len() + 1; }
    }
    println!("{:?}", hm);
    println!("{}", ans);
}

fn divide(n: usize) -> Option<usize> {
    for i in 2..(n / 2) + 1 {
        if n % i == 0 {
            return Some(i);
        }
    }
    return None;
}

fn prime_factor(n: usize) -> Vec<usize> {
    let mut x = n;
    let mut vec = Vec::new();
    while let Some(f) = divide(x) {
        x /= f;
        vec.push(f);
    }
    vec.push(x);
    return vec;
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
