use std::io;
use std::cmp::Ordering;


fn main() {
    let (a, b) = {
        let i = read::<isize>();
        (i[0], i[1])
    };
    let op = match a.cmp(&b) {
        Ordering::Less => "<",
        Ordering::Equal => "==",
        Ordering::Greater => ">"
    };
    println!("a {} b", op);
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
