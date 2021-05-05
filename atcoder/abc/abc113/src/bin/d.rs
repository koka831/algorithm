use std::io;


/// right: (k - 1) + (h - (k - 1)) / 2
/// left:  (h - (k - 1)) / 2
fn main() {
    const MOD: usize = 1_000_000_007;
    let (h, w, k) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };

    let r = (k - 1) + (h - (k - 1)) / 2;
    let l = (h - (k - 1)) / 2;

    println!("{} {}", l, r);

    let mut dp = vec![vec![vec![0; w]; w]; w];


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
