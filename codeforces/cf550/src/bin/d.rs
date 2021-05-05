use std::io;
use std::collections::HashMap;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();

    let mut hm = HashMap::new();

    for i in 0..n {
        *hm.entry(an[i]).or_insert(0) += 1;
    }

    let mut id = 0;
    for i in 0..n {
        if hm.get(&an[i]).unwrap() >= hm.get(&an[id]).unwrap() {
            id = i;
        }
    }

    let mut cnt = 0;

    let max = an[id];
    for a in an.iter() { if &max != a { cnt += 1; }}

    println!("{}", cnt);
    let mut l = id;
    while l > 0 {
        if an[l - 1] < max {
            println!("1 {} {}", l, l + 1);
        } else if an[l - 1] > max {
            println!("2 {} {}", l, l + 1);
        }
        l -= 1;
    }
    let mut r = id;
    while r < n - 1 {
        if an[r + 1] < max {
            println!("1 {} {}", r + 2, r + 1);
        } else if an[r + 1] > max {
            println!("2 {} {}", r + 2, r + 1);
        }
        r += 1;
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
