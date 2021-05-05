use std::io;
use std::cmp;

fn main() {
    let _n = read_one::<usize>();
    let xn = read::<isize>();
    let min = *xn.iter().min().unwrap();
    let max = *xn.iter().max().unwrap();

    let mut ans = std::isize::MAX;
    for i in min..=max {
        let mut cost = 0;
        for x in xn.iter() {
            cost += (x - i as isize).pow(2);
        }
        ans = cmp::min(ans, cost);
    }

    println!("{}", ans);
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
