use std::io;

fn main() {
    loop {
        let (n, x) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        if n + x == 0 { return; }
        else { println!("{}", solve(n, x)); }
    }
}

fn solve(n: usize, x: usize) -> usize {
    let mut cnt = 0;
    for i in 1..=n { for j in i+1..=n { for k in j+1..=n {
        if i + j + k == x { cnt += 1; }
    }}}
    cnt
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
