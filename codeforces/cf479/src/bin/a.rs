use std::io;


/// https://codeforces.com/problemset/problem/977/A
fn main() {
    let (mut n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    for _ in 0..k {
        if n % 10 == 0 { n /= 10; }
        else { n -= 1; }
    }

    println!("{}", n);
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
