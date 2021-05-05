use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    if k % 2 != 0 {
        println!("{}", (n / k).pow(3));
    } else {
        let mut cnt: usize = 0;
        for i in 0..n + 1 {
            if i % k == k / 2 { cnt += 1; }
        }
        println!("{}", (n / k).pow(3) + cnt.pow(3));
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
