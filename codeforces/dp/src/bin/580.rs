use std::io;
use std::cmp;


/// Kefa and First Steps
fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    let mut r = 0;
    let mut ans = 0;

    for l in 0..n {
        while r < n && (r <= l || an[r - 1] <= an[r]) { r += 1; }
        ans = cmp::max(ans, r - l);

        if l == r { r += 1; }
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
