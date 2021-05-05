use std::io;
use std::cmp;


fn main() {
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut cnt = 0;
    let mut len = 0;
    for i in 0..s.len() {
        if ['A', 'C', 'G', 'T'].contains(&s[i]) {
            len += 1;
            cnt = cmp::max(cnt, len);
        } else { len = 0; }
    }

    println!("{}", cnt);
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
