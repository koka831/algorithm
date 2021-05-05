use std::io;


fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();

    for _ in 0..n {
        let i = read::<usize>();
        ab.push((i[0], i[1]));
    }

    ab.sort_by_key(|x| x.1);
    ab.reverse();

    let mut sum = 0;
    for x in ab.iter() {
        sum += x.0 * x.1;
    }


    println!("{}", sum + ab[0].1 * x);
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
