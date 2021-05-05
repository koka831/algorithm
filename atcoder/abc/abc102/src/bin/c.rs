use std::io;
use std::cmp;

fn main() {
    let n = read_one::<usize>();
    let mut a = read::<isize>();

    for i in 0..n {
        a[i] -= (i + 1) as isize;
    }

    /* mean
    let d: isize = a.iter().sum();
    let b = d / n as isize;

    let mut ans = 100_000_000_000;
    for j in 0..5 {
        let mut s = 0;
        for k in 0..n {
            s += (a[k] - (b - (j - 2))).abs();
        }
        ans = cmp::min(ans, s);
    }
    println!("{}", ans);
    */

    /* median
    let median;
    let mut b = a.clone();
    b.sort();
    if n % 2 == 0 {
        median = (b[n / 2] + b[(n - 1)/ 2]) / 2;
    } else {
        median = b[(n - 1) / 2];
    }

    let mut ans = 100_000_000_000;
    for j in 0..5 {
        let mut s = 0;
        for k in 0..n {
            s += (a[k] - (median - (j - 2))).abs();
        }
        ans = cmp::min(ans, s);

    }
    println!("{}", ans);
    */
    a.sort();
    let mut ans = 0;
    let b = as[ n / 2];
    for i in 0..n {
        ans += (a[i] - b).abs();
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
