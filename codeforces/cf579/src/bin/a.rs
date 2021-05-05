use std::io;
use std::collections::VecDeque;


fn main() {
    let n = read_one::<usize>();
    for _ in 0..n {
        let k = read_one::<usize>();
        let pn = read::<usize>();
        let mut kn = pn.clone();
        kn.sort();
        let mut ln = kn.clone();
        ln.reverse();

        let mut pn = VecDeque::from(pn);

        let mut flg = false;
        for _ in 0..k {
            let f = pn.pop_front().unwrap();
            pn.push_back(f);
            if pn == kn || pn == ln { flg = true; }
        }

        if flg { println!("YES"); }
        else { println!("NO"); }
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
