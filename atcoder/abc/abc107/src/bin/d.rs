use std::io;


fn main() {
    let n = read_one::<usize>();
    let an = read::<usize>();
    let mut count = 0;
    let mut ans = 0;
    let med = (n + 1) / 2;
    for i in 1..n + 1 {
        if i <= (n + 1) / 2 {
            println!("{}'s diff: {}", i, i * 2 - 1);
            count += i * 2 - 1;
        } else {
            let diff = med * 2 - (i - med - (n + 1) % 2) * 2;
            println!("{}'s diff: {}", i, diff);
            count += diff;
        }
        println!("count: {}", count);
        if count >= ((n * (n + 1)) / 4 + 1) {
            ans = an[i - 1];
            println!("{}", ans);
            break;
        }
    }
    println!("middle id: {}", (n * (n + 1)) / 4 + 1);
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
