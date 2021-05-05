use std::io;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();

    let mut before = vec![0; n];
    let mut after = vec![0; n];
    for i in 0..n { for j in 0..n {
        if an[i] > an[j] {
            if i > j { before[i] += 1; }
            else { after[i] += 1; }
        }
    }}

    let mut ans: usize = 0;
    for i in 0..n {
        ans += factorial(before[i], k - 1) + factorial(after[i], k);
        ans %= MOD;
    }

    println!("{}", ans);
}

fn factorial(n: usize, k: usize) -> usize {
    n * (k * (k - 1) % MOD) % MOD
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
