use std::io;
use std::cmp;
use std::collections::*;


fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let n = read_one::<isize>();
    let (d, x) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    let mut total = x;
    for i in 0..n {
        let a = read_one::<isize>();
        total += 1 + ((d - 1) / a);
    }


    println!("{}", total);
}
