use std::io;

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

    let n = read_one::<usize>();

    let mut x = read::<usize>();
    let y = x.clone();

    x.sort();

    let large = x[n/2];
    let small = x[(n-2)/2]; 

    for i in 0..n {
        if y[i] <= small {
            println!("{}", x[n/2]);
        } else if y[i] >= large {
            println!("{}", x[n/2 - 1]);
        }
    }
}
