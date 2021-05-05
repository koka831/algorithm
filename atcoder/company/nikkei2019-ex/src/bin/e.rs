use std::io;


fn main() {
    let n = read_one::<usize>();
    let chr = vec!["a", "b", "c", "d", "e"];
    for i in 1..n + 1 {
        let mut buf = String::new();
        for j in 0..5 {
            if i % (j + 2) == 0 {
                buf += chr[j];
            }
        }
        if buf.len() == 0 { buf = i.to_string(); }
        println!("{}", buf);
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
