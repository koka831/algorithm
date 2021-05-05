use std::io;


fn main() {
    let mut x = read_one::<usize>();
    loop {
        if is_prime(x) {
            println!("{}", x);
            return;
        }
        x += 1;
    }
}

fn is_prime(n: usize) -> bool {
    if n == 2 { return true; }
    for i in 2..(n as f64).sqrt().ceil() as usize + 1 {
        if n % i == 0 { return false; }
    }
    true
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
