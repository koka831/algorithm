use std::io;


fn main() {
    let n = read_one::<usize>();
    let ans = recur(3, n) + recur(5, n) + recur(7, n);
    println!("{}", ans);
}

fn recur(n: usize, th: usize) -> usize {
    if n > th { return 0; }
    let s = n.to_string();
    let mut ans = 1;
    if !(s.contains("3") && s.contains("5") && s.contains("7")) {
        ans = 0;
    } 
    return ans
        + recur(n * 10 + 3, th)
        + recur(n * 10 + 5, th)
        + recur(n * 10 + 7, th);
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
