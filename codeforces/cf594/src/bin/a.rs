use std::io;


fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let n = read_one::<u64>();
    let pn = read::<u64>();
    let cnt_p: u64 = pn.into_iter().filter(|&p| p % 2 == 0).count() as u64;
    let m = read_one::<u64>();
    let qm = read::<u64>();
    let cnt_q: u64 = qm.into_iter().filter(|&q| q % 2 == 0).count() as u64;
    println!("{}", cnt_p * cnt_q + (n - cnt_p) * (m - cnt_q));
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
