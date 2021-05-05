use std::io;

fn main() {
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut cnt = 0;
    for i in 0..s.len() {
        if i > s.len() - 3 { break; }
        if s[i] == 'Z' && s[i + 1] == 'O' && s[i + 2] == 'N' && s[i + 3] == 'e' {
            cnt += 1;
        }
    }

    println!("{}", cnt);
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
