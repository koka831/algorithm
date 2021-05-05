use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        let i = read_one::<String>().chars().collect::<Vec<char>>();
        vec.push(i);
    }

    println!("{:?}", vec);

    let mut dp = vec![vec![0; m]; n];
    for i in 0..m { for j in 0..n {

    }}
    let mut ans = 0;
    for i in 0..m { for j in 0..n {
    }}
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
