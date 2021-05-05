use std::io;


fn main() {
    let p = read_one::<usize>();
    let mut x = 1789997546303usize; // P = 1000
    for _ in 0..1000 - p {
        if x % 2 == 0 { x /= 2; }
        else { x = x * 3 + 1; }
    }
    println!("{}", x);
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
