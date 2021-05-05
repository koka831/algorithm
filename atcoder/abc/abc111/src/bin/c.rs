use std::io;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let n = read_one::<usize>();
    let vn = read::<usize>();
    let mut odd = HashMap::new();
    let mut even = HashMap::new();

    for i in 0..n {
        if i % 2 == 0 {
            *even.entry(vn[i]).or_insert(0) += 1;
        } else {
            *odd.entry(vn[i]).or_insert(0) += 1;
        }
    }
    let mut even_key = 0;
    let mut odd_key = 0;
    let mut even_max = 0;
    let mut even_2nd = 0;
    let mut odd_max = 0;
    let mut odd_2nd = 0;
    for (k, v) in even.iter() {
        if *v >= even_2nd {
            if *v >= even_max {
                even_key = *k;
                even_2nd = even_max;
                even_max = *v;
            } else {
                even_2nd = *v;
            }
        }
    }

    for (k, v) in odd.iter() {
        if *v >= odd_2nd {
            if *v >= odd_max {
                odd_key = *k;
                odd_2nd = odd_max;
                odd_max = *v;
            } else {
                odd_2nd = *v;
            }
        }
    }

    let diff = if odd_key == even_key {
        cmp::max(odd_2nd + even_max, odd_max + even_2nd)
    } else {
        odd_max + even_max
    };

    let cnt = n - diff;
    println!("{}", cnt);
}


#[allow(unused)]
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


#[allow(unused)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
