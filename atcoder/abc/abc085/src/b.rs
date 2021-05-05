use std::io;
use std::collections::BTreeSet;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


fn main() {
    let n = read::<usize>()[0];
    let mochi = (0..n).map(|_| read::<usize>()[0])
        .collect::<BTreeSet<_>>();

    println!("{:?}", mochi);
    println!("{}", mochi.len());
}
