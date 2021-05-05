use std::io;


fn main() {
    let n = read_one::<usize>();

    for i in 1..n + 1 {
        let a = i;
        for j in 1..n + 1 {
            let b = a * j;
            if a * b > n && b / a < n {
                println!("{} {}", b, a);
                return;
            }
        }
    }
    println!("-1");
    return;
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
