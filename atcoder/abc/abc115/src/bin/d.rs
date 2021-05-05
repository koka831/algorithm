use std::io;


fn main() {
    let (n, x) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    println!("{}", recur(n, x));

}

fn recur(n: usize, x: usize) -> usize {
    if n == 0 {
        if x == 0 { return 0; }
        else { return 1; }
    }

    let mut h = 1;
    let mut l = 1;
    for _ in 0..n {
        h = h * 2 + 3;
        l = l * 2 + 1;
    }
    l = (l - 1) / 2;

    if x == 1 { 0 }
    else if x <= h / 2 {
        recur(n - 1, x - 1)
    } else if x == h / 2 + 1 {
        recur(n - 1, x - 1) + 1
    } else {
        recur(n - 1, x - h / 2 - 1) + l + 1
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
