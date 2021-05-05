use std::io;
use std::collections::HashMap;


static MOD : u64 = (1e9 as u64) + 7;

fn main() {
    let (n, m) = {
        let i = read::<u64>();
        (i[0], i[1])
    };

    // O(\sqrt(n) * log(n))
    let mut hm = HashMap::new();
    let mut x = m;
    while let Some(f) = divide(x) {
        x /= f;
        *hm.entry(f).or_insert(0) += 1;
    }

    let mut ans = 1;
    for v in hm.values() {
        ans = (ans * ncr(v + n - 1, *v)) % MOD;
    }
    println!("{}", ans);
}


fn ncr(n: u64, r: u64) -> u64 {
    match (n, r) {
        (0, _) | (_, 0) => 1,
        _ => ((ncr(n, r - 1) * (n - r + 1) / r)) % MOD,
    }
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
