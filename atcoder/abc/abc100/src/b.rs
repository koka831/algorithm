use std::io;

fn main() {
    let (d, n) = {
        let i = read::<i32>();
        (i[0], i[1])
    };

    let mut ans = (100f64.powi(d) as i32) * n;
    if n == 100 {
        if d == 0 { ans = 101; }
        if d == 1 { ans = 10100; }
        if d == 2 { ans = 1010000; }
    }
    println!("{}", ans);
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
