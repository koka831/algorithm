use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut pn = read::<usize>();
    pn.sort();
    let cnt_a = pn.iter().filter(|p| **p <= a).count();
    let cnt_c = pn.iter().filter(|p| **p > b).count();
    let cnt_b = n - cnt_a - cnt_c;
    println!("{}", cmp::min(cnt_a, cmp::min(cnt_b, cnt_c)));
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
