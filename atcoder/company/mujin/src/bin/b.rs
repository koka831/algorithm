use std::io;


fn main() {
    let a = read_one::<isize>();
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut cnt = a;
    let mut flg = false;
    for c in s {
        if cnt == 0 { flg = true; }
        if c == '+' { cnt += 1; }
        else { cnt -= 1; }
        if cnt == 0 { flg = true; }
    }

    if flg { println!("Yes"); }
    else { println!("No"); }
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
