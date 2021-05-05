use std::io;
use std::cmp;

fn main() {
    let (n, x, y) = {
        let i = read::<usize>();
        (i[0], i[1] as isize - 1, i[2] as isize - 1)
    };

    let mut cost = vec![0; n];
    for i in 0..n { for j in (i + 1)..n {
        let (i, j) = (i as isize, j as isize);

        let c1 = (x - i).abs() + (y - j).abs();
        let c2 = (x - j).abs() + (y - i).abs();
        let c = cmp::min(j - i, cmp::min(c1 + 1, c2 + 1)) as usize;
        cost[c] += 1;
    }}

    for i in 1..n {
        println!("{}", cost[i]);
    }
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
