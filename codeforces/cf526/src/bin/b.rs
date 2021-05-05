use std::io;


/// s < diff => min
/// s > diff => min - ceil(diff / n)
fn main() {
    let (n, _) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vn = read::<usize>();
    vn.sort();
    vn.reverse();
    let mut amount = vec![0; n + 1];
    amount[0] = 0;
    for i in 1..n {
        amount[i] = amount[i - 1] + (vn[i - 1] - vn[i]) * i;
    }
    amount[n] = amount[n - 1] + vn[n - 1] * n;

    for _ in 0..n {
        // if amount[]
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
