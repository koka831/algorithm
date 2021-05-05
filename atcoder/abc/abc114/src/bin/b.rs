use std::io;


fn main() {
    let s = read_one::<String>().chars().collect::<Vec<char>>();
    let mut diff = 1000000;
    for i in 0..s.len() - 2 {
        let x = s[i].to_digit(10).unwrap() * 100 + s[i + 1].to_digit(10).unwrap() * 10 + s[i + 2].to_digit(10).unwrap();
        if ((x as isize) - 753).abs() <= diff {
            diff = ((x as isize) - 753).abs();
        }
    }
    println!("{}", diff);
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
