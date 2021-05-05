use std::io;

fn main() {
    let (n, d) = {
        let i = read::<usize>();
        (i[0], i[1] as isize)
    };
    let d2 = d * d;
    let mut cnt = 0;
    for _ in 0..n {
        let (x, y) = {
            let i = read::<isize>();
            (i[0], i[1])
        };
        if x * x + y * y <= d2 { cnt += 1; }
    }

    println!("{}", cnt);
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
