use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
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

fn main() {
    let n = read_one::<usize>();

    let prime = [2, 3, 13, 23, 43, 53, 73, 83, 103, 113, 163, 173, 233,
                 263, 283, 293, 313, 353, 373, 383, 433, 443, 463, 503,
                 523, 563, 593, 613, 643, 653, 673, 683, 733, 743, 773,
                 823, 883, 953, 983, 1013, 1033, 1063, 1093, 1103, 1123,
                 1153, 1163, 1193, 1213, 1223, 1303, 1373, 1423, 1433,
                 1453, 1483, 1493, 1523, 1613, 1733, 1873, 2213];

    println!("{}", prime.len());

    let s = &prime[0..n].iter().fold(String::from(""), |a, b| a + &*b.to_string() + " ");
    println!("{}", s.trim());
}
