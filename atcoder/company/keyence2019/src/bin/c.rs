use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let bn = read::<usize>();

    if an.iter().sum::<usize>() < bn.iter().sum::<usize>() {
        println!("-1");
        return;
    }

    let mut diff = vec![0; n];
    for i in 0..n {
        diff[i] = an[i] as isize - bn[i] as isize;
    }

    diff.sort();
    diff.reverse();

    let mut cnt = 0;
    let mut sum: isize = 0;

    for i in 0..n {
        if an[i] < bn[i] {
            cnt += 1;
            sum += (bn[i] - an[i]) as isize;
        }
    }

    let mut idx = 0;
    while sum > 0 {
        sum -= diff[idx] as isize;
        idx += 1;
    }

    println!("{}", idx + cnt);
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
