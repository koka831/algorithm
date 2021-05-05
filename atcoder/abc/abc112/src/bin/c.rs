use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut vec = Vec::new();
    for _ in 0..n {
        let i = read::<isize>();
        vec.push((i[0], i[1], i[2]));
    }

    vec.sort_by_key(|x| x.2);
    vec.reverse();

    let mut ans_x = 0;
    let mut ans_y = 0;
    let mut ans_h = 0;
    for x in 0..101 { for y in 0..101 {
        let mut flg = true;
        let h = vec[0].2 + (x - vec[0].0).abs() + (y - vec[0].1).abs();
        for v in vec.iter() {
            if v.2 != cmp::max(h - (x - v.0).abs() - (y - v.1).abs(), 0) { flg = false; continue; }
        }

        if flg {
            ans_x = x;
            ans_y = y;
            ans_h = h;
        }
    }}

    println!("{} {} {}", ans_x, ans_y, ans_h);
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
