use std::io;
use std::collections::HashMap;


fn main() {
    let _ = read_one::<usize>();
    let mut an = read::<usize>();
    an.sort();

    let mut hm = HashMap::new();

    for a in an.iter() {
        *hm.entry(a).or_insert(0) += 1;
    }

    for h in hm.values() { if *h > 2 {
        println!("NO"); return;
    }}

    let mut inc = Vec::new();
    let mut dec = Vec::new();

    inc.push(an[0]);
    for i in 1..an.len() {
        if an[i] == an[i - 1] { dec.push(an[i]); }
        else { inc.push(an[i]); }
    }
    dec.reverse();
    println!("YES");
    println!("{}", inc.len());
    println!("{}", inc.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", dec.len());
    println!("{}", dec.iter().map(|&i| i.to_string()).collect::<Vec<String>>().join(" "));
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
