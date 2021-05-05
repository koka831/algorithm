use std::io;
use std::cmp;
use std::collections::HashMap;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut hm = HashMap::new();

    for _ in 0..n {
        let (t, d) = {
            let i = read::<usize>();
            (i[0], i[1])
        };

        if !hm.contains_key(&t) {
            hm.insert(t, vec![d]);
        } else {
            hm.get_mut(&t).unwrap().push(t);
        }
    }

    let mut cumsum = HashMap::new();
    for v in hm.values_mut() {
        v.sort();
        v.reverse();
        for j in 0..v.len() {
            cumsum
        }
    }

    let mut ans = 0;
    for i in 1..k + 1 {
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
