use std::io;


fn main() {
    let n = read_one::<usize>();
    let pn = read::<usize>();
    let mut cnt = 0;
    for i in 1..(n - 1) {
        if pn[i - 1] < pn[i] && pn[i] < pn[i + 1] { cnt += 1; }
        if pn[i - 1] > pn[i] && pn[i] > pn[i + 1] { cnt += 1; }
    }
    println!("{}", cnt);
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
