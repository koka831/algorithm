use std::io;

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
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut vec = Vec::new();
    for _ in 0..h {
        let chr: Vec<char> = read_one::<String>()
            .chars().collect();
        vec.push(chr);
    }

    let mut flg = true;
    for j in 0..h { for k in 0..w {
        if vec[j][k] == '#' {
            let mut f = false;
            if 0 < j && vec[j - 1][k] == '#' { f = true; }
            if 0 < k && vec[j][k - 1] == '#' { f = true; }
            if j < h - 1 && vec[j + 1][k] == '#' { f = true; }
            if k < w - 1 && vec[j][k + 1] == '#' { f = true; }
            if !f { flg = false; break; }
        }
    }}

    if flg {
        println!("Yes");
    } else {
        println!("No"); 
    }
}
