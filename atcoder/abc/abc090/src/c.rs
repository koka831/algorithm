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

    let (n, m) = {
        let a = read::<isize>();
        (a[0], a[1])
    };

    if n == 1 && m == 1 {
        println!("1");
    } else if n < 2 || m < 2 {
        println!("{}", (cmp::max(n - 2, m - 2)));
    } else {
        println!("{}", (cmp::max(n - 2, 0)) * (cmp::max(m - 2, 0)));
    }

}
