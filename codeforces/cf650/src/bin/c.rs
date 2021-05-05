use std::io;

fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let s = read_one::<String>().chars().collect::<Vec<_>>();

    let mut runlen = Vec::new();
    let mut cnt = 1;

    for i in 1..n {
        if s[i] != s[i - 1] { runlen.push(cnt); cnt = 0; }
        cnt += 1;
    }
    runlen.push(cnt);

    let mut ans = 0;
    for i in 0..runlen.len() {
        if s[0] == '0' {
            if i % 2 != 0 { continue; }
        } else { // s[0] == '1'
            if i % 2 == 0 { continue; }
        }

        if i == 0 || i == runlen.len() - 1 {
            ans += runlen[i] / (k + 1);
        } else {
            ans += (runlen[i] - k) / (k + 1);
        }
    }

    if runlen.len() == 1 && s[0] == '0' {
        ans = (runlen[0] - 1) / (k + 1) + 1;
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
