use std::io;

fn main() {
    let (k, n) = {
        let i = read::<usize>();
        (i[0] as isize, i[1])
    };
    let an = read::<isize>();
    let mut ans = 0;
    let mut cost = 0;
    for i in 1..n {
        ans += an[i] - an[i - 1];
        cost = cost.max((an[i] % k - an[i - 1] % k).abs());
    }

    let lo = ((k - an[n - 1]).abs() + an[0]) % k;
    ans += lo;
    cost = cost.max(lo);

    println!("{}", ans - cost);
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
