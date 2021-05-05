use std::io;


fn main() {
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let (r, s, p) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let tn = read_one::<String>().chars().collect::<Vec<_>>();
    let mut ans = 0;
    let mut ok = vec![false; n];
    for i in 0..n {
        if i >= k && tn[i - k] == tn[i] {
            if ok[i - k] {
                ok[i] = false;
            } else {
                ok[i] = true;
                continue;
            }
        }
        ans += match tn[i] {
            'r' => p,
            's' => r,
            'p' => s,
            _   => unreachable!()
        };
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
