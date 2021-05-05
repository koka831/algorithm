use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut ab = Vec::new();

    for _ in 0..n {
        let (a, b) = {
            let i = read::<i64>();
            (i[0], i[1])
        };
        ab.push((a, b, a + b));
    }

    let mut ans: i64 = 0;
    ab.sort_by_key(|a| a.2);

    for _ in 0..n {
        if ab.len() == 0 { break; }
        let choku = ab.pop().unwrap();
        ans += choku.0;

        if ab.len() == 0 { break; }
        let aoki = ab.pop().unwrap();
        ans -= aoki.1;
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
