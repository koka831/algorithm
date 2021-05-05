use std::io;


fn main() {
    let n = read_one::<usize>();

    let mut w = Vec::new();
    for _ in 0..n {
        let s = read_one::<String>();
        w.push(s.chars().collect::<Vec<char>>());
    }

    let mut ans = true;
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if w[i] == w[j] { ans = false; }
        }
        if i > 0 {
            if w[i][0] != w[i - 1][w[i - 1].len() - 1] { ans = false; }
        }
    }

    if ans { println!("Yes"); }
    else { println!("No"); }
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
