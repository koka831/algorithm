
use std::io;


fn main() {
    let s = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let k = read_one::<usize>();
    let mut ans = 0;
    let mut cnt = 0;
    for i in s {
        if i == 1 { cnt += 1;}
        if k <= cnt { ans = i; break; }
        if i != 1 { ans = i; break; }
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
