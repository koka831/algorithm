use std::io;
use std::cmp;


fn main() {
    let _ = read_one::<usize>();
    let an = read::<usize>();
    let mut odd = Vec::new();
    let mut even = Vec::new();
    for a in an {
        if a % 2 == 0 { even.push(a); }
        else { odd.push(a); }
    }

    even.sort();
    odd.sort();

    let siz = cmp::min(odd.len(), even.len());

    for _ in 0..siz { even.pop(); odd.pop(); }
    let ans = even.iter().sum::<usize>() + odd.iter().sum::<usize>() - cmp::max(even.iter().max().unwrap_or(&0), odd.iter().max().unwrap_or(&0));
    println!("{}", ans);
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
