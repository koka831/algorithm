use std::io;
use std::cmp;


fn main() {
    let (n, k, m) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };
    let an = read::<isize>().iter().sum::<isize>();
    let x = n * m - an;
    if x > k { println!("-1"); }
    else { println!("{}", cmp::max(x, 0)); }
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
