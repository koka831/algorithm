use std::io;


fn main() {
    let n = read_one::<usize>();
    let s = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as u8 as usize)
        .collect::<Vec<_>>();
    println!("{:?}", s);

    let mut cnt = 0;
    for i in 0..1000 {

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
