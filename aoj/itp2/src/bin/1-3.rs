use std::io;


fn main() {
    let q = read_one::<usize>();
    let mut vec = Vec::new();
    let mut cur = 0;
    for _ in 0..q {
        let s = read::<usize>();
        match s[0] {
            0 => {
                vec.insert(cur, s[1]);
                cur += 1;
            },
            1 => {
                cur += s[1];
            },
            _ => {
                match vec.remove(s[1]) {
                    Some(_) => {},
                    None => { cur = 0; }
                }
            }
        }
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
