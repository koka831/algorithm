use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut hn = read::<usize>();
    hn.reverse();
    let mut flg = true;
    let mut prev = hn[0];
    for i in 1..n {
        if prev + 1 == hn[i] { prev = hn[i] - 1; }
        else if prev == hn[i] { prev = hn[i]; }
        else if prev > hn[i] { prev = hn[i]; }
        else { flg = false; break; }
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
