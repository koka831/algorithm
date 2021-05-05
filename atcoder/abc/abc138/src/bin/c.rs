use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut vn = read::<usize>();
    vn.sort();
    let mut ans = 0f64;
    ans += vn[0] as f64 / (2f64 * (n - 1) as f64);
    for i in 1..n {
        ans += vn[i] as f64 / (2f64 * (n - i) as f64);
    }
    println!("{}", ans);

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
