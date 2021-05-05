use std::io;


fn main() {
    let (w, h, x, y, r) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3], i[4])
    };
    let res = if x >= r && y >= r && x + r <= w && y + r <= h {
        "Yes"
    } else { "No" };
    println!("{}", res);
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
