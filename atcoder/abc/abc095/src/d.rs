#![deny(unused_imports)]
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

    let (n, c) = {
        let i = read::<i64>();
        (i[0], i[1])
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        let inp = read::<i64>();
        vec.push(inp);
    }
    vec.sort_by(|a, b| a[0].cmp(&b[0]) );

    println!("{:?}", vec);

    let mut score = 0;
    let mut pos = 0;
    let mut prev_choice = 0;

    while vec.len() > 0 {
        let len = &vec.len() - 1;
        { // for lifetime
        let left = &vec[len];
        let right = &vec[0];

        let score_l = left[1] - (pos - (c - left[0])).abs();
        let score_r = right[1] - (pos - right[0]).abs();
        if cmp::max(score_l, score_r) >= 0 {
            if score_l > score_r {
                score += score_l;
                pos = left[0];
                prev_choice = len;
                println!("added: {}", score_l);
            } else {
                score += score_r;
                pos = right[0];
                prev_choice = 0;
                println!("added: {}", score_r);
            }
        } else {
            println!("break");
            break;
        }
        } // end of lifetime
        println!("{}", pos);
        vec.remove(prev_choice);
        println!("{:?}", vec);
    }

    println!("{}", score);
}
