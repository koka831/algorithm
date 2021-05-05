use std::io;


#[allow(dead_code)]
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

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let n = read_one::<isize>();
    let mut a = read::<f64>();

    a.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut b = a.clone();
    let x = a[(n - 1) as usize];
    let half :f64= x / 2f64;
    b.sort_by(|a, b| {
        (half - a).abs().partial_cmp(&(half - b).abs()).unwrap()
    });

    println!("{} {}", x, b[0]);
}
