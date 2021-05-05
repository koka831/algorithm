use std::io;
use std::cmp;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut g = vec![Vec::new(); h];
    for _ in 0..h { g.push(read_one::<String>().chars().collect::<Vec<_>>()); }
    let mut l = vec![vec![0; w]; h];
    let mut r = vec![vec![0; w]; h];
    let mut u = vec![vec![0; w]; h];
    let mut d = vec![vec![0; w]; h];

    for i in 0..h { for j in 0..w {
        if i > 0 {
            if g[i][j] == '.' { l[i][j] = l[i][j - 1] + 1; }
            else { l[i][j] = l[i][j]; }
        }
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
