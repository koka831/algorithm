use std::io;
use std::cmp;


fn main() {
    let (_, _, x, y) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    let xn = read::<isize>();
    let ym = read::<isize>();
    let xmax = xn.iter().max().unwrap();
    let ymin = ym.iter().min().unwrap();

    if cmp::max(x, *xmax) < cmp::min(y, *ymin) { println!("No War"); }
    else { println!("War"); }
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
