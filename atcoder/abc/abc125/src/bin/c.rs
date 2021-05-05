use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let mut l = vec![0; n + 1];
    let mut r = vec![0; n + 1];

    for i in 0..n {
        l[i + 1] = gcd(l[i], an[i]);
    }
    for i in (0..n).rev() {
        r[i] = gcd(r[i + 1], an[i]);
    }

    let ans = (0..n)
        .map(|i| gcd(l[i], r[i + 1]))
        .max()
        .unwrap();
    println!("{}", ans);
}

fn gcd(p: usize, q: usize) -> usize {
    if q == 0 { p }
    else { gcd(q, p % q) }
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
