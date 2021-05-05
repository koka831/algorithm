use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut an = Vec::new();
    for _ in 0..n {
        an.push(read_one::<usize>());
    }

    if an.iter().all(|a| a % 2 == 0) {
        println!("second");
    } else {
        println!("first");
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
