use std::io;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q { solve(); }
}

fn solve() {
    let _ = read_one::<usize>();
    let an = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as usize).collect::<Vec<usize>>();
    let bn = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as usize).collect::<Vec<usize>>();
    let curve = [3, 4, 5, 6];
    let mut top = true;
    let mut ans = true;
    for (a, b) in an.iter().zip(bn) {
        if top {
            if curve.contains(&a) && [1, 2].contains(&b) {
                ans = false;
                break;
            }
            if curve.contains(&a) { top = !top; }
        } else {
            if curve.contains(&b) && [1, 2].contains(&a) {
                ans = false;
                break;
            }
            if curve.contains(&b) { top = !top; }

        }
    }

    if ans && (!top) { println!("YES"); }
    else { println!("NO"); }
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
