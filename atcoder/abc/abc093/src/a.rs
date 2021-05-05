use std::io;
use std::collections::BTreeSet;

fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let a = read_one::<String>();
    let mut bt = BTreeSet::new();
    for s in a.chars() {
        bt.insert(s);
    }
    if bt.len() == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
