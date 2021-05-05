use std::io;


fn main() {
    let l = read_one::<usize>();
    let mut al = Vec::new();
    for _ in 0..l {
        al.push(read_one::<isize>());
    }
    println!();

    for i in 0..l - 1 {
        println!("{}", (al[i] - al[i + 1]).abs());
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
