use std::io;


fn main() {
    let (n, t) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ct = Vec::new();
    for _ in 0..n {
        let i = read::<usize>();
        ct.push((i[0], i[1]));
    }

    let ans = ct.iter()
        .filter(|x| x.1 <= t)
        .map(|x| x.0)
        .min();

    match ans {
        Some(x) => println!("{}", x),
        None => println!("TLE"),
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
