use std::io;


fn main() {
    let (_, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<usize>();
    an.sort();

    let ans = if k == 0 { an[0] - 1 } else { an[k - 1] };

    let mut cnt = 0;
    for a in an {
        if a <= ans { cnt += 1; }
    }

    if cnt != k || !(1 <= ans && ans <= 10e9 as usize) {
        println!("-1"); return;
    }
    println!("{}", ans);
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
