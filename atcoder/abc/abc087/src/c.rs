use std::io;
use std::cmp;

#[allow(dead_code)]
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


#[allow(dead_code)]
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

    let mut vec = Vec::new();
    for _ in 0..2 {
        vec.push(read::<usize>());
    }
    // println!("{:?}", vec);
    let mut score = 0;
    for i in 1..n {
        score = cmp::max(&vec[0][0..i].iter().sum::<usize>() + &vec[1][cmp::max(i - 1, 0)..n].iter().sum::<usize>(), score);
    }

    println!("{}", score);
}
