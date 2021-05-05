use std::io;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut s = read_one::<String>().chars().collect::<Vec<_>>();
    s.reverse();
    let mut path = Vec::new();
    let mut idx = 0;
    while idx < n {
        if (1..m + 1).all(|i| s[idx + i] == '1') { println!("-1"); return; }
        for i in (0..m + 1).rev() {
            if idx + i > n { continue; }
            if s[idx + i] == '0' { idx += i; path.push(i); break; }
        }
    }
    path.reverse();
    for i in 0..path.len() {
        if i == path.len() - 1 { println!("{}", path[i]); }
        else { print!("{} ", path[i]); }
    }
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
