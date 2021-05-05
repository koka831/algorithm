use std::io;


fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let t = read_one::<String>().chars().collect::<Vec<char>>();

    if s.len() != t.len() { println!("No"); return; }

    let vowel = ['a', 'e', 'i', 'o', 'u'];
    for i in 0..s.len() {
        if vowel.contains(&s[i]) && !vowel.contains(&t[i]) {
            println!("No"); return;
        } else if !vowel.contains(&s[i]) && vowel.contains(&t[i]) {
            println!("No"); return;
        }
    }
    println!("Yes");
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
