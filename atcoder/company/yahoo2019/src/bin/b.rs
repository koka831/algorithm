use std::io;


fn main() {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    for _ in 0..3 {
        let ab = read::<usize>();
        for i in ab {
            match i {
                1 => a += 1,
                2 => b += 1,
                3 => c += 1,
                4 => d += 1,
                _ => unreachable!(),
            }
        }
    }

    if a > 2 || b > 2 || c > 2 || d > 2 {
        println!("NO");
    } else {
        println!("YES");
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
