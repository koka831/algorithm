use std::io;


fn main() {
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();
    let mut is_yymm = false;
    let mut is_mmyy = false;
    if s[0] <= '1' && s[1] <= '2' || s[0] < '1' { is_mmyy = true; }
    if s[2] <= '1' && s[3] <= '2' || s[2] < '1' { is_yymm = true; }
    if s[0] == '0' && s[1] == '0' { is_mmyy = false; }
    if s[2] == '0' && s[3] == '0' { is_yymm = false; }
    match (is_yymm, is_mmyy) {
        (true, true) => println!("AMBIGUOUS"),
        (true, false) => println!("YYMM"),
        (false, true) => println!("MMYY"),
        _ => println!("NA"),
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
