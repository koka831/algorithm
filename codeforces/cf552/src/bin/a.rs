use std::io;


fn main() {
    let mut xs = read::<usize>();
    xs.sort();

    let max = xs.iter().max().unwrap();

    println!("{} {} {}",
        xs[0] + xs[1] - max,
        xs[1] + xs[2] - max,
        xs[2] + xs[0] - max,
    );
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
