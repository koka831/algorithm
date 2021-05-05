use std::io;
use std::cmp;
use std::collections::HashMap;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let mut s = vec![0; n + 1];
    s[1] = an[0];
    for i in 2..n + 1 {
        s[i] = s[i - 1] + an[i - 1];
    }

    let mut si = Vec::new();
    for i in 0..n + 1 {
        si.push((s[i] - i) % k);
    }

    let mut cnt = HashMap::new();
    for i in 0..cmp::min(n + 1, k - 1) {
        *cnt.entry(si[i]).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for i in 0..n + 1 {
        *cnt.entry(si[i]).or_insert(0) -= 1;
        if i + k <= n + 1 {
            *cnt.entry(si[i + k - 1]).or_insert(0) += 1;
        }
        ans += cmp::max(*cnt.get(&si[i]).unwrap_or(&0), 0);
    }

    println!("{}", ans);
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
