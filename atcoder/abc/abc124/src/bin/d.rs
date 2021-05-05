use std::io;
use std::cmp;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut counts = Vec::new();
    let mut cnt = 1;
    for i in 1..n {
        if s[i - 1] != s[i] {
            counts.push(cnt);
            cnt = 1;
        } else {
            cnt += 1;
        }
    }
    counts.push(cnt);
    let len = counts.len();

    let mut acc = Vec::new();
    acc.push(0);
    for i in 0..len {
        acc.push(acc[i] + counts[i]);
    }

    let mut ans = 0;
    for i in 0..len {
        let to = cmp::min(i + k + 1, len);
        println!("{}:{}", i, to);
        ans = cmp::max(ans, acc[to] - acc[i]);
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
