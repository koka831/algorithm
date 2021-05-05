use std::io;


fn main() {
    let n = read_one::<usize>();
    let num = [111, 222, 333, 444, 555, 666, 777, 888, 999];
    for i in num.iter() {
        if n <= *i { println!("{}", i); break; }
    }
}


#[allow(unused)]
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


#[allow(unused)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
