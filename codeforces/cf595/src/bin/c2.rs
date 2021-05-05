use std::io;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let n = read_one::<u64>();
    let mut x = n;
    let mut buf = Vec::new();
    while x > 0 {
        let y = x % 3;
        buf.push(y);
        x /= 3;
    }
    // println!("{:?}", buf);
    if buf.contains(&2) {
        println!("{}", 3u64.pow(buf.len() as u32));
    } else {
        println!("{}", n);
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
