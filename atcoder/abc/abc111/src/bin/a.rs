use std::io;


fn main() {
    let n = read_one::<String>();
    let mut buf = Vec::new();
    for c in n.chars() {
        match c {
            '1' => buf.push('9'),
            '9' => buf.push('1'),
            _ => buf.push(c),
        }
    }
    for c in buf.iter() {
        print!("{}", c);
    }
    println!();
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
