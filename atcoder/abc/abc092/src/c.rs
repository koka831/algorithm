use std::io;
use std::cmp;


fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {
    let n = read_one::<usize>();
    let path = read::<isize>();
    // cost[n] = cost of path(n - 1) to path(n)
    let mut cost = Vec::new();

    for i in 0..(n + 1) {
        if i == 0 {
            cost.push(path[0].abs());
        } else if i == n {
            cost.push(path[n - 1].abs());
        } else {
            cost.push((path[i - 1] - path[i]).abs());
        }
    }

    let sum: isize = cost.iter().sum();

    for i in 1..(n + 1) {
        let cost_i = sum - cost[i - 1] - cost[i];
        if i == 1 {
            println!("{}", cost_i + path[i].abs());
        } else if i == n {
            println!("{}", cost_i + path[n - 2].abs());
        } else {
            println!("{}", cost_i + (path[i - 2] - path[i]).abs());
        }
    }
}
