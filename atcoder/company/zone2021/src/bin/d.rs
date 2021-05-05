use std::io;
use std::collections::VecDeque;

fn main() {
    let s = read_one::<String>().chars().collect::<Vec<_>>();
    let mut reversed = false;

    let mut t = VecDeque::new();
    for c in s {
        if c == 'R' {
            reversed = !reversed;
        } else {
            if reversed {
                if t.len() > 0 && c == t[0] {
                    t.pop_front();
                } else {
                    t.push_front(c);
                }
            } else {
                let len = t.len();
                if len > 0 && c == t[len - 1] {
                    t.pop_back();
                } else {
                    t.push_back(c);
                }
            }
        }
    }

    if reversed {
        for c in t.iter().rev() {
            print!("{}", c);
        }
    } else {
        for c in t {
            print!("{}", c);
        }
    }

    println!();
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
