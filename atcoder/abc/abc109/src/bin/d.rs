use std::io;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut vec = Vec::new();
    for _ in 0..h {
        let i = read::<usize>();
        vec.push(i);
    }

    let mut ss = Vec::new();
    for i in 0..h { for j in 0..w {
        if vec[i][j] % 2 != 0 {
            if j < w - 1 {
                vec[i][j + 1] += 1;
                vec[i][j] -= 1;
                ss.push(format!("{} {} {} {}", i + 1, j + 1, i + 1, j + 2));
            }
            if j == w - 1 && i < h - 1 {
                vec[i + 1][j] += 1; vec[i][j] -= 1;
                ss.push(format!("{} {} {} {}", i + 1, j + 1, i + 2, j + 1));
            }
        }
    }}

    println!("{}", ss.len());
    for s in ss {
        println!("{}", s);
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
