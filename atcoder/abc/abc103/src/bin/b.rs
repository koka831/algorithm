use std::io;

fn main() {
    let s = read_one::<String>();
    let b = read_one::<String>().chars().collect::<Vec<char>>();
    let mut flg = false;
    for i in 0..s.len() {
        let (prev, past) = b.split_at(i);
        let x = past.into_iter().collect::<String>();
        let y = prev.into_iter().collect::<String>();
        if s == x + &y {
            flg = true;
        }
    }

    if flg { println!("Yes"); }
    else { println!("No"); }
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
