use std::io;
use std::collections::VecDeque;


fn main() {
    let q = read_one::<usize>();
    let mut vec = VecDeque::new();
    for _ in 0..q {
        let s = read::<isize>();
        match s[0] {
            0 => {
                if s[1] == 0 { vec.push_front(s[2]); }
                else { vec.push_back(s[2]); }
            },
            1 => { println!("{}", vec[s[1] as usize]); },
            _   => {
                if s[1] == 0 { vec.pop_front(); }
                else { vec.pop_back(); }
            },
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
