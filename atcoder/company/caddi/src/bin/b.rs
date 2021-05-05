use std::io;


fn main() {
    let (n, h, w) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut ans = 0;
    for _ in 0..n {
        let (h_i, w_i) = {
            let i = read::<usize>();
            (i[0], i[1])
        };

        if h_i >= h && w_i >= w { ans += 1; }
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
