use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }

    ab.sort_by_key(|x| x.0);
    ab.reverse();
    let mut count: usize = 0;
    let mut price: usize = 0;
    loop {
        let x = ab.pop().unwrap();
        if count + x.1 >= m {
            println!("{}", price + (m - count) * x.0);
            return;
        }
        count += x.1;
        price += x.1 * x.0;
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
