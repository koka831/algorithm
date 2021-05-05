use std::io;


fn main() {
    let (a, b, c, d) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3])
    };
    println!("{}", b - a - (calc(b, c, d) - calc(a - 1, c, d)) + 1);
}

fn calc(n: usize, a: usize, b: usize) -> usize {
    let da: usize = n / a;
    let db: usize = n / b;
    da + db - n / lcm(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
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
