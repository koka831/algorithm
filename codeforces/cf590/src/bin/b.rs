use std::io;
use std::collections::{VecDeque, BTreeSet};


fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let ids = read::<usize>();
    let mut v = VecDeque::new();
    let mut s = BTreeSet::new();

    for i in ids.into_iter() {
        if !s.contains(&i) { // O(n) → O(log n)
            if s.len() >= k {
                let c = v.pop_back().unwrap();
                s.remove(&c);
            }
            v.push_front(i);
            s.insert(i);
        }
    }

    println!("{}", v.len());
    for i in 0..v.len() {
        print!("{} ", v[i]);
    }
    println!();
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
