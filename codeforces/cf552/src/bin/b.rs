use std::io;
use std::cmp;


fn main() {
    let _ = read_one::<usize>();
    let mut an = read::<isize>();
    an.sort();
    an.dedup();

    if an.len() > 3 { println!("-1"); return; }
    if an.len() == 1 { println!("0"); return; }
    if an.len() == 2 {
        if (an[1] - an[0]) % 2 == 0 {
            println!("{}", cmp::min((an[1] - an[0]) / 2, an[1] - an[0]));
        } else {
            println!("{}", an[1] - an[0]);
        }
        return;
    }
    if an.len() == 3 {
        if an[2] - an[1] == an[1] - an[0] {
            println!("{}", an[1] - an[0]);
        } else {
            println!("-1");
        }
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
