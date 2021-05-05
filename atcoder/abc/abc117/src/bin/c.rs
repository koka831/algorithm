use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut xm = read::<isize>();
    xm.sort();
    let mut cost = Vec::new();
    for i in 0..m - 1 {
        cost.push((xm[i] - xm[i + 1]).abs() as usize);
    }

    cost.sort();

    for _ in 0..n - 1 { cost.pop(); }
    println!("{}", cost.into_iter().sum::<usize>());
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
