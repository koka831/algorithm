use std::io;
use std::cmp;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut score = Vec::new();
    for _ in 0..n { score.push(read::<usize>()); }

    let mut ans: usize = 0;
    for i in 0..m {
        for j in i..m {
            let mut sum: usize = 0;
            for s in score.iter() {
                sum += cmp::max(s[i], s[j]);
            }
            ans = cmp::max(ans, sum);
        }
    }

    println!("{}", ans);
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
