use std::io;


fn main() {
    let (mut a, mut b) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut ans = 0;
    if a > b {
        ans += a; a -= 1;
    } else {
        ans += b; b -= 1;
    }

    if a > b {
        ans += a;
    } else {
        ans += b;
    }
    println!("{}", ans);
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
