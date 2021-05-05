use std::io;
use std::cmp;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<usize>();
    let mut an = Vec::new();
    for _ in 0..n {
        an.push(read_one::<isize>());
    }

    an.sort();

    let a = diff(an.clone());
    an.reverse();
    let b = diff(an.clone());
    println!("{}", cmp::max(a, b));
}


fn diff(mut an: Vec<isize>) -> usize {

    let mut n = an.len();
    let mut vd = VecDeque::new();
    vd.push_front(an[0]);
    an.remove(0);
    n -= 1;
    for i in 0..n {
        match i % 4 {
            0 => {
                vd.push_back(an[n - i / 2 - 1]);
            },
            1 => {
                vd.push_front(an[n - i / 2 - 2]);
            },
            2 => {
                vd.push_back(an[i / 2 - 1]);
            },
            3 => {
                vd.push_front(an[i / 2]);
            },
            _ => unreachable!(),
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += (vd[i] - vd[i + 1]).abs();
    }

    ans as usize
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
