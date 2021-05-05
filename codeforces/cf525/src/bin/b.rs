use std::io;


fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut an = read::<usize>();
    an.push(0);
    an.sort();
    an.dedup();
    let x = an.len();

    for i in 0..k {
        if i < x - 1 {
            println!("{}", an[i + 1] - an[i]);
        } else {
            println!("0");
        }
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
