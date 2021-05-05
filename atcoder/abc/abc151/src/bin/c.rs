use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut wa = vec![0; n];
    let mut ac = vec![false; n];
    for _ in 0..m {
        let (p, s) = {
            let i = read::<String>();
            (i[0].clone().parse::<usize>().unwrap() - 1, i[1].clone())
        };

        if ac[p] { continue; }
        if s == "WA" { wa[p] += 1; }
        else { ac[p] = true; }
    }
    let sum_ac = ac.iter().map(|&t| if t { 1 } else { 0 }).sum::<usize>();
    let mut sum_wa = 0;
    for i in 0..n {
        if ac[i] {
            sum_wa += wa[i];
        }
    }
    println!("{} {}", sum_ac, sum_wa);
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
