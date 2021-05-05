use std::io;
use std::collections::HashMap;


fn main() {
    let n = read_one::<usize>();
    for _ in 0..n {
        let mut s = read_one::<String>()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();
        s.sort();

        let mut ans = "Yes";
        let mut hm = HashMap::new();
        *hm.entry(s[0]).or_insert(0) += 1;

        for i in 1..s.len() {
            if let Some(_) = hm.get(&s[i]) { ans = "No"; break; };
            *hm.entry(s[i]).or_insert(0) += 1;
            if s[i - 1] + 1u8 != s[i] { ans = "No"; break; }
        }
        println!("{}", ans);
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
