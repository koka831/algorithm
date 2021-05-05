use std::io;


fn main() {
    let _ = read_one::<usize>();
    let mut cs = read_one::<String>().chars()
        .collect::<Vec<char>>();
    cs.reverse();
    let mut stack = Vec::new();
    let mut poped = Vec::new();
    let mut cnt = 0;
    while let Some(c) = cs.pop() {
        if c == '(' { stack.push(c); }
        else {
            if stack.last() == Some(&'(') { stack.pop(); }
            else { // swap ')'
                poped.push(c);
            }
        }
        if stack.len() == 0 { cnt += 1; }
    }

    println!("{}", cnt);
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
