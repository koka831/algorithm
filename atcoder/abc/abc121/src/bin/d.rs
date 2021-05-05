use std::io;
use std::cmp;


fn main() {
    let (a, b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let sum_a = xorsum(a);
    let sum_b = xorsum(b);
}

fn xorsum(x: usize) -> usize {
    let mut v = Vec::new();
    let mut a: usize = 1;
    loop {
        if a > x { break; }
        if a == 1 {
            if x % 4 == 1 || x % 4 == 2 { v.push(1); } else { v.push(0); }
        }
        else {
            let pos: usize = (x + 1) % (a * 2);
            println!("{}", pos);
            let b = if pos >= a && pos % 2 != 0 { 1 } else { 0 };
            v.push(b);
        }
        a *= 2;
    }
    v.reverse();
    println!("{:?}", v);
    0
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
