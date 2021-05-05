use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let wn = read::<isize>();
    let mut ans = 1_000_000_000;
    for i in 0..n {
        let c = wn[0..i].iter().sum::<isize>() - wn[i..n].iter().sum::<isize>();
        ans = cmp::min(ans, c.abs());
    }
    println!("{}", ans);
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
