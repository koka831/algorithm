use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut xy = Vec::new();
    for _ in 0..n {
        xy.push(read::<isize>());
    }
    let mut xy_clone = xy.clone();
    xy_clone.sort_by_key(|v| v[0].abs() + v[1].abs());
    xy_clone.reverse();

    // mod xy.max()
    let dist = xy_clone[0][0].abs() + xy_clone[0][1].abs();
    for x in xy.iter() {
        if (x[0].abs() + x[1].abs()) % 2 != dist % 2 {
            println!("-1");
            return;
        }
    }

    // only 300 score
    let m = dist;
    println!("{}", m);
    print!("1");
    for _ in 0..m - 1 {
        print!(" 1");
    }
    println!();
    for x in xy.iter() {
        let mut remains = m;
        let mut buf = String::new();
        // LR
        if x[0] > 0 {
            for _ in 0..x[0] {
                buf += "R";
                remains -= 1;
            }
        } else {
            for _ in 0..x[0].abs() {
                buf += "L";
                remains -= 1;
            }
        }

        // UD
        if x[1] > 0 {
            for _ in 0..x[1] {
                buf += "U";
                remains -= 1;
            }
        } else {
            for _ in 0..x[1].abs() {
                buf += "D";
                remains -= 1;
            }
        }

        // remain
        for _ in 0..remains / 2 {
            buf += "UD";
        }
        println!("{}", buf);
    }
}


#[allow(unused)]
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


#[allow(unused)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
