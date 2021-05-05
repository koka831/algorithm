use std::io;
use std::cmp;


fn main() {
    let (a, b, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let mut cnt = 0;

    for i in (1..cmp::min(a, b) + 1).rev() {
        if a % i == 0 && b % i == 0 { cnt += 1; }
        if cnt == k {
            println!("{}", i);
            return;
        }
    }
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
