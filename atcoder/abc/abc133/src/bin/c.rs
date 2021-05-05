use std::io;
use std::cmp;


fn main() {
    let (l, r) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ans: usize = std::usize::MAX;
    'outer: for i in l..r + 1 { for j in (i + 1)..r + 1 {
        ans = cmp::min(ans, (i * j) % 2019);
        if ans == 0 { break 'outer; }
    }}

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
