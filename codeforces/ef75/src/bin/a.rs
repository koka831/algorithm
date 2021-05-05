use std::io;


fn main() {
    let t = read_one::<usize>();
    for _ in 0..t { solve(); }
}

fn solve() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut cnt = 1;
    let mut buf = Vec::new();
    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] { cnt += 1; }
        else {
            if cnt % 2 != 0 {
                for i in 0..buf.len() {
                    if buf[i] == s[i] { buf.remove(i); break; }
                }
            } else {
                buf.push(s[i]);
            }
            cnt = 1;
        }
    }
    println!("{:?}", buf);
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
