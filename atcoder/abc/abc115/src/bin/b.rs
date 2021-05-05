use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut ans = 0;
    let mut sum = 0;
    for _ in 0..n {
        let p = read_one::<usize>();
        sum += p;
        ans = cmp::max(ans, p);
    }
    println!("{}", sum - ans / 2);
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
