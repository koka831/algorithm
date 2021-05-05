use std::io;


fn main() {
    let (n, m, c) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };
    let bm = read::<isize>();
    let mut an = Vec::new();
    for _ in 0..n {
        an.push(read::<isize>());
    }

    let mut cnt = 0;
    for i in 0..n as usize {
        let mut sum = c;
        for j in 0..m as usize {
            sum += an[i][j] * bm[j];
        }
        if sum > 0 { cnt += 1; }
    }

    println!("{}", cnt);
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
