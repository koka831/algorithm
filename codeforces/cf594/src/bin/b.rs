use std::io;
use std::iter::FromIterator;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<u64>();
    let mut an = read::<u64>();
    an.sort();
    let mut q = VecDeque::from_iter(an.iter());
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    for i in 0..n {
        if i % 2 == 0 {
            x += q.pop_back().unwrap();
        } else {
            y += q.pop_front().unwrap();
        }
    }

    println!("{}:{}", x, y);
    println!("{}", x*x + y*y);
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
