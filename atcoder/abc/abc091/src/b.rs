use std::io;
use std::cmp;
use std::collections::*;


fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {
    let n = read::<usize>()[0];
    let mut n_map = HashMap::new();
    for _ in 0..n {
        let si = read_one::<String>();
        *n_map.entry(si).or_insert(0) += 1;
    }

    let m = read::<usize>()[0];
    let mut m_map = HashMap::new();
    for _ in 0..m {
        let ti = read_one::<String>();
        *m_map.entry(ti).or_insert(0) += 1;
    }

    let mut max = 0;

    for x in &n_map {
        let y = match m_map.get(x.0) {
            Some(v) => *v,
            None => 0,
        };

        max = cmp::max(max, x.1 - y);

    }

    println!("{}", max);
}
