use std::io;


fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1] - 1)
    };
    let mut s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();
    let c = match s[k] {
        'A' => 'a',
        'B' => 'b',
        'C' => 'c',
        _ => unreachable!(),
    };
    s[k] = c;
    for c in s {
        print!("{}", c);
    }
    println!();
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
