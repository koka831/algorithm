use std::io;


fn main() {
    let n = read_one::<usize>();
    let (t, a) = {
        let i = read::<f64>();
        (i[0], i[1])
    };
    let hn = read::<f64>();
    let mut idx = 0;
    let mut min = 1000000000000.0;
    for i in 0..n {
        let _t = ((t - hn[i] * 0.006) - a).abs();
        if _t <= min {
            idx = i;
            min = _t;
        }
    }
    println!("{}", idx + 1);
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
