use std::io;


fn main() {
    let n = read_one::<usize>();
    let dn = read::<usize>();
    let mut ans: usize = 0;
    for i in 0..n { for j in 0..n {
        if i == j { continue; }
        ans += dn[i] * dn[j];
    }}
    println!("{}", ans / 2);
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
