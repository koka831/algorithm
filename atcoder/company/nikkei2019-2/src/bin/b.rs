use std::io;
use std::collections::HashMap;


fn main() {
    let _ = read_one::<usize>();
    let dn = read::<usize>();
    let mut hm = HashMap::new();
    for d in dn.iter() {
        *hm.entry(d).or_insert(0) += 1;
    }

    if dn[0] != 0 { println!("0"); return; }
    //
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
