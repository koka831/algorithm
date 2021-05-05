use std::io;
use std::cmp;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut xn = read::<isize>();
    xn.sort();
    let mut start = ::std::usize::MAX;
    for i in 0..n {
        start = cmp::min(start, xn[i].abs() as usize);
    }
    let mut ans = ::std::usize::MAX;
    for i in 0..n - k + 1 {
        ans = cmp::min(ans, xn[i].abs() as usize + (xn[i] - xn[i + k - 1]).abs() as usize);
        ans = cmp::min(ans, xn[i + k - 1].abs() as usize + (xn[i] - xn[i + k - 1]).abs() as usize);
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
