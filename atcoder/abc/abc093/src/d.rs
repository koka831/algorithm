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
    let n = read_one::<usize>();
    println!("{}", n);
    for i in 0..n {
        let (a, b) = {
            let num = read::<usize>();
            (num[0], num[1])
        };
        let max = a * b;
        let mut pair = 0;
        for j in 1..max {
            if j < max / j { pair += 2; }
            else { break; }
        }
        pair -= 1;
        println!("{}", pair);
    }
}
