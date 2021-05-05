use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let s = read_one::<String>().chars().collect::<Vec<char>>();

    let mut cnt = 0;
    for i in 0..n {
        let mut a = s.clone();
        a.split_off(i);
        let b = &s[i..n];
        let mut cnt_i = 0;

        a.sort();
        a.dedup();

        for c in a {
            if b.contains(&c) {
                cnt_i += 1;
            }
        }
        cnt = cmp::max(cnt, cnt_i);
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
