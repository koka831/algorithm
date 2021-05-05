use std::io;


fn main() {
    let n = read_one::<usize>();
    for _ in 0..n {
        solve();
    }
}

fn solve() {
    let (x, y) = {
        let i = read::<u64>();
        // println!("{:?}", i);
        (i[0], i[1])
    };
    if x - y != 1 { println!("yes"); }
    else { println!("no"); }
}


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
std::str::FromStr,
T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split_whitespace()
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
