use std::io;
use std::cmp;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {

    let (n, m, x) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };

    let a = read::<isize>();

    let mut cost_0 = 0;
    let mut cost_n = 0;

    for i in a {
        if i < x { cost_0 += 1; }
        else { cost_n += 1; }
    }

    println!("{}", cmp::min(cost_0, cost_n));
}
