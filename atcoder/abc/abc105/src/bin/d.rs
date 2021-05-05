use std::io;
use std::collections::HashMap;

type Table = HashMap<usize, usize>;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let a = read::<usize>();
    let mut sums = vec![0; a.len() + 1];
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
        sums[i + 1] = sum;
    }


    let mut ans = 0;
    for l in 0..n {
        let mut sum = 0;
        for r in l..n {
        sum += a[r];
        if sum % m == 0 { ans += 1; }
        }
    }

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
