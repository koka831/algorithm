use std::io;
use std::cmp;
use std::collections::*;


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

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {
    let (a, b) = {
        i = read::<usize>();
        (i[0], i[1])
    };


    let mut vec: Vec<Vec<i8>> = Vec::new();
    for i in 0..cmp::max(a, b) {
        if i % 2 == 0 {
            // 最初に反転した0101010のvec<vec>を作成
            // w = A, b = Bになるようにマス目を反転させてく
            // passしたらbreak, print
        }
    }
}
