use std::io;
use std::cmp;


fn main() {
    let (n, k, m) = {
        let i = read::<usize>();
        (i[0], i[1], i[2])
    };
    let mut an = read::<isize>();
    an.sort();
    an.reverse();

    let mut operation = m as isize;

    if k < m {
        for _ in 0..cmp::min(m - k, n - 1) {
            an.pop();
            operation -= 1;
        }
    }

    an.reverse();

    for i in 0..an.len() - 1 {
        let mut cost = cmp::min(an[i + 1] - an[i], k as isize);
        if operation < cost {
            cost = operation;
        }

        operation -= cost;
        if cost > 0 { an[i + 1] = an[i]; }
        else { an[i + 1] = an[i] - cost; }
        if operation < 0 { break; }
    }

    if operation > 0 {
        println!("{}", an[0] as usize + cmp::min(operation as usize, k) / an.len());
    } else {
        println!("{}", an.iter().sum::<isize>());
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
