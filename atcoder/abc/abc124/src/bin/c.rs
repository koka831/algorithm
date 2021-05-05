use std::io;
use std::cmp;


fn main() {
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut ans = 0;
    // 0101
    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == '0' { ans += 1; }
        } else {
            if s[i] == '1' { ans += 1; }
        }
    }

    let mut ansb = 0;
    // 0101
    for i in 0..s.len() {
        if i % 2 != 0 {
            if s[i] == '0' { ansb += 1; }
        } else {
            if s[i] == '1' { ansb += 1; }
        }
    }
    println!("{}", cmp::min(ans, ansb));
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
