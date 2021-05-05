use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>().iter()
        .map(|a| a - 1)
        .collect::<Vec<_>>();
    let mut cnt = 0;
    for (i, &a) in an.iter().enumerate() {
        if a != i - cnt { cnt += 1; }
    }
    if cnt == n { println!("-1"); }
    else { println!("{}", cnt); }
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
