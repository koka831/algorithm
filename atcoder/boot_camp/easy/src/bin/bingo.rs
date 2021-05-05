use std::io;

fn main() {
    let mut card = Vec::new();
    let mut flag = vec![false; 9];
    for _ in 0..3 { card.push(read::<usize>()); }
    let n = read_one::<usize>();
    for _ in 0..n {
        let b = read_one::<usize>();
        for i in 0..3 {
            for j in 0..3 {
                if card[i][j] == b { flag[i + j * 3] = true; }
            }
        }
    }

    let bingo = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];

    for i in 0..bingo.len() {
        if bingo[i].iter().all(|&j| flag[j]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
