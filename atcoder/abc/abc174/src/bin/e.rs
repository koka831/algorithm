use std::io;

fn main() {
    let (_n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<usize>();
    an.sort_by(|a, b| b.cmp(a));

    let mut ok = *an.iter().max().unwrap();
    let mut ng = 0;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        if satisfy(&an, k, m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    println!("{}", ok);
}

fn satisfy(logs: &[usize], k: usize, x: usize) -> bool {
    let mut cnt = 0;

    for &log in logs {
        if log <= x || cnt > k { break; }

        cnt += (log as f64 / x as f64).ceil() as usize - 1;
    }

    cnt <= k
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
