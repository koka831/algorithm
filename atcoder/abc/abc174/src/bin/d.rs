use std::io;

fn main() {
    let _n = read_one::<usize>();
    let cn = read_one::<String>().chars().collect::<Vec<_>>();
    let r = cn.iter().filter(|&c| *c == 'R').count();
    let w = (0..r).filter(|&i| cn[i] == 'W').count();

    println!("{}", w);
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
