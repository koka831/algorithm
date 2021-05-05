use std::io;

fn main() {
    let (w, h, x, y) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3])
    };
    let is_center = if x * 2 == w && y * 2 == h { 1 } else { 0 };
    println!("{} {}", (w * h) as f32 / 2f32, is_center);
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
