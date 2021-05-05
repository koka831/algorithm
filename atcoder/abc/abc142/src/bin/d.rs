use std::io;
use std::collections::HashSet;


fn main() {
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let cs = factor(gcd(a, b));
    println!("{}", cs.len());

}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a }
    else { gcd(b, a % b) }
}


fn divide(n: usize) -> Option<usize> {
    for i in 2..(n as f64).sqrt().ceil() as usize + 1 {
        if n % i == 0 { return Some(i); }
    }
    None
}

fn factor(n: usize) -> HashSet<usize> {
    let mut x = n;
    let mut fs = HashSet::new();
    fs.insert(1);
    while let Some(f) = divide(x) {
        x /= f;
        fs.insert(f);
    }
    fs.insert(x);
    fs
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
