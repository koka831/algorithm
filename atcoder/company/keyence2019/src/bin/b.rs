use std::io;


fn main() {
    let s = read_one::<String>();
    let key = "keyence".to_string();
    for i in 0..s.len() { for j in i..s.len() {
        let f = &s[0..i];
        let e = &s[j..s.len()];
        if key == format!("{}{}", f, e) {
            println!("YES");
            return;
        }
    }}
    println!("NO");
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
