use std::io;
use std::cmp;


fn main() {
    let (n, m, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let an = read::<usize>();
    let bm = read::<usize>();

    let mut an_sum = vec![0usize];
    let mut bm_sum = vec![0usize];

    for i in 0..n { an_sum.push(an_sum[i] + an[i]); }
    for i in 0..m { bm_sum.push(bm_sum[i] + bm[i]); }

    let mut ans: usize = 0;
    let mut j = m;

    for i in 0..=n {
        if an_sum[i] > k { break; }
        while bm_sum[j] > k - an_sum[i] { j -= 1; }
        ans = cmp::max(ans, i + j);
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
