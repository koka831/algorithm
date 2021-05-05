use std::io;
use itertools::Itertools;


fn main() {
    let s = read_one::<String>().chars()
        .collect::<Vec<_>>();
    let n = s.len();

    let mut i = 0;
    let mut cnt = vec![0; n];
    while i < n {
        let l = i;
        while i + 1 < n && s[i] == s[i + 1] { i += 1; }

        if s[i] == 'R' {
            cnt[i] += (i - l + 2) / 2;
            cnt[i + 1] += (i - l + 1) / 2;
        } else {
            cnt[l] += (i - l + 2) / 2;
            cnt[l - 1] += (i - l + 1) / 2;
        }
        i += 1;
    }

    println!("{}", cnt.iter().format(" "));
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
