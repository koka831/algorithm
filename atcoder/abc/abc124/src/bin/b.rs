use std::cmp;
use std::io;

fn main() {
    let _ = read_one::<usize>();
    let hn = read::<usize>();
    let mut mx = 0;
    let mut ans = 0;
    for h in hn {
        if mx <= h {
            ans += 1;
            mx = cmp::max(mx, h);
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
