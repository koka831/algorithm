use std::io;


fn main() {
    let (k, a, b) = {
        let i = read::<u64>();
        (i[0], i[1], i[2])
    };

    if a >= b || b - a <= 1{
        println!("{}", k + 1);
        return;
    } 

    let ans: u64 = if (k + 1 - a) % 2 == 0 {
        (b - a) * (k + 1 - a) / 2 + a
    } else {
        (b - a) * (k - a) / 2 + a + 1
    };

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
