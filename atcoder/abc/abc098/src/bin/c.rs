use std::io;

fn main() {
    let n = read_one::<usize>();
    let s = read_one::<String>();
    let s = s.chars()
        .map(|c| match c {
            'E' => 1,
            'W' => 0,
            _   => 2,
        }).collect::<Vec<usize>>();

    let mut sum_r = s.iter().sum::<usize>();
    let mut sum_l = 0;
    let mut sum = sum_r + sum_l;
    for i in 0..n {
        match s[i] {
            0 => {
                sum_l += 1;
            },
            1 => {
                sum_r -= 1;
            }
            _ => {},
        }
        if sum_l + sum_r < sum { sum = sum_l + sum_r; }
    }
    println!("{}", sum);
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
