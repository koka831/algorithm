use std::io;
use std::cmp;


fn main() {
    let sn = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let tn = read_one::<String>().chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();

    let mut dp = vec![vec![0; 201]; 201];

    for i in 0..sn.len() {
        for j in i..tn.len() {
            if sn[i] == tn[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1
            } else {
                dp[i + 1][j + 1] = cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }

    println!("{}", sn.len() - dp.iter().flatten().max().unwrap());
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
