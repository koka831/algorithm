use std::io;


fn main() {
    let _ = read_one::<usize>();
    let ln = read::<usize>();
    let max = ln.iter().max().unwrap();
    let sum = ln.iter().sum::<usize>() - max;
    if max < &sum {
        println!("Yes");
    } else {
        println!("No");
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
