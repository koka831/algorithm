use std::io;


fn main() {
    let (n, q) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let s = read_one::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut cnt = vec![0; n];

    for i in 1..n {
        if s[i - 1] == 'A' && s[i] == 'C' { cnt[i] += 1; }
        cnt[i] += cnt[i - 1];
    }

    for _ in 0..q {
        let (l, r) = {
            let i = read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        println!("{}", cnt[r] - cnt[l])
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
