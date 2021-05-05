use std::io;


fn main() {
    let (n, m) = {
        let i = read::<u64>();
        (i[0], i[1])
    };

    let mut ans = 1;

    if n == 1 {
        println!("{}", m);
        return;
    }

    for i in 1..(m / n + 1) {
        if m % i == 0 { ans = i; }
    }

    println!("{}", ans);
}


#[allow(dead_code)]
fn ncr(n: u64, r: u64) -> u64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ((ncr(n, r - 1) * (n - r + 1) / r))
    }
}


#[allow(dead_code)]
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
