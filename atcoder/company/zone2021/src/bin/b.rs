use std::io;

fn main() {
    let (n, d, h) = {
        let i = read::<usize>();
        (i[0], i[1] as f64, i[2] as f64)
    };

    let mut ans = 0f64;
    for _ in 0..n {
        let (di, hi) = {
            let i = read::<f64>();
            (i[0], i[1])
        };

        ans = ans.max(hi - di * (h - hi) / (d - di));
    }

    println!("{}", ans.max(0f64));
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
