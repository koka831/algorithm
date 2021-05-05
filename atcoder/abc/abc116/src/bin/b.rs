use std::io;


fn main() {
    let s = read_one::<usize>();
    let mut buf = Vec::new();
    let mut prev = s;
    for i in 1..1000000 {
        let v;
        if i == 1 { v = s; }
        else {
            if prev % 2 == 0 { v = prev / 2; }
            else { v = prev * 3 + 1; }
        };

        if buf.contains(&v) {
            println!("{}", i);
            return;
        }
        buf.push(v);
        prev = v;
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
