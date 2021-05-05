use std::io;
use std::cmp;


fn main() {
    let (n, m) = {
        let i = read::<isize>();
        (i[0], i[1])
    };

    let mut l = 0;
    let mut r = n;

    for _ in 0..m {
        let (a, b) = {
            let i = read::<isize>();
            (i[0], i[1])
        };

        l = cmp::max(l, a);
        r = cmp::min(r, b);
    }

    println!("{}", cmp::max(0, r - l + 1));
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
