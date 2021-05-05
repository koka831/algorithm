use std::io;


fn main() {
    let (h, w) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut aw = Vec::new();
    for _ in 0..h {
        let i = read_one::<String>().chars().collect::<Vec<char>>();
        aw.push(i);
    }

    let mut bw = aw.clone();
    for i in (0..h).rev() {
        let mut flg = true;
        for j in (0..w).rev() {
            if aw[i][j] != '.' { flg = false; }
        }
        if flg { bw.remove(i); }
    }

    let mut cw = bw.clone();
    for i in (0..bw[0].len()).rev() {
        let mut flg = true;
        for j in (0..bw.len()).rev() {
            if bw[j][i] != '.' { flg = false; }
        }
        if flg {
            for k in 0..bw.len() {
                cw[k].remove(i);
            }
        }
    }
    for row in cw {
        for c in row {
            print!("{}", c);
        }
        println!();
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
