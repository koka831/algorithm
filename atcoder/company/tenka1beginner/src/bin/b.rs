use std::io;


fn main() {
    let (mut a, mut b, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    for i in 0..k {
        if i % 2 == 0 { // takahashi
            if a % 2 != 0 {
                a -= 1;
            }
            b += a / 2;
            a /= 2;
        } else { // aoki
            if b % 2 != 0 {
                b -= 1;
            }
            a += b / 2;
            b /= 2;
        }
    }
    println!("{} {}", a, b);
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
