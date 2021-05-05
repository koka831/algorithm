use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut pros = 0.0;
    for i in 1..n + 1 {
        let mut ans = i;
        let mut time = 1;
        while ans < k { time *= 2; ans *= 2; }
        pros += (1.0 / (time as f64)) / n as f64;
    }

    println!("{}", pros);
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
