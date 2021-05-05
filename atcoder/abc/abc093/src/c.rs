use std::io;
use std::cmp;

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
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let mut num = read::<usize>();
    num.sort();

    let d_a = num[2] - num[1];
    let d_b = num[2] - num[0];

    match ((d_a % 2 == 0), (d_b % 2 == 0)) {
        (true, true) => println!("{}", (d_a + d_b) / 2),
        (false, false) => {
            let d_a = d_a - 1;
            let d_b = d_b - 1;
            println!("{}", 1 + (d_a + d_b) / 2);
        },
        (false, true) => {
            let d_a = d_a + 1;
            println!("{}", 1 + (d_a + d_b) / 2);
        },
        (true, false) => {
             let d_b = d_b + 1;
            println!("{}", 1 + (d_a + d_b) / 2);
        }
    }
}
