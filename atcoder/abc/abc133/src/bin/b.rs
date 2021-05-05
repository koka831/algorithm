use std::io;


fn main() {
    let (n, d) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(read::<isize>());
    }

    let mut cnt = 0;
    for i in 0..n { for j in (i + 1)..n {
        let mut norm = 0;
        for k in 0..d {
            norm += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
        }

        let mut f = false;
        for k in 0..norm + 1 {
            if k * k == norm { f = true; }
        }

        if f { cnt += 1; }
    }}

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
