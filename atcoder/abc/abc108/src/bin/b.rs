use std::io;


fn main() {
    let xy = read::<isize>();
    let dx = xy[2] - xy[0];
    let dy = xy[3] - xy[1];
    println!("{} {} {} {}", xy[2] - dy, xy[3] + dx, xy[0] - dy, xy[1] + dx);
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
