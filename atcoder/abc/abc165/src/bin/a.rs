use std::io;

fn main() {
    let k = read_one::<usize>();
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    for i in 0..=1000 {
        if a <= i * k && i * k <= b { println!("OK"); return; }
    }

    println!("NG");
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

#[allow(unused)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(unused)]
fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}
