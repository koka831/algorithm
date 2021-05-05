use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let a = read_one::<String>().chars().collect::<Vec<char>>();
    let b = read_one::<String>().chars().collect::<Vec<char>>();
    let c = read_one::<String>().chars().collect::<Vec<char>>();

    let mut cnt = 0;
    for i in 0..n {
        let mut diff_a = 0;
        let mut diff_b = 0;
        let mut diff_c = 0;

        if a[i] != b[i] { diff_a += 1; diff_b += 1; }
        if b[i] != c[i] { diff_b += 1; diff_c += 1; }
        if c[i] != a[i] { diff_c += 1; diff_a += 1; }
        cnt += cmp::min(diff_a, cmp::min(diff_b, diff_c));
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
