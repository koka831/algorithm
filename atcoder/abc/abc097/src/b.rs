use std::io;
use std::cmp;

fn main() {
    let x = read_one::<usize>();

    if x == 1 {
        println!("1");
    } else {
        let mut max = 0;

        for i in 2..32 {
            let b = i;
            let mut p = 2;
            let mut c = b;
            while c <= x {
                c = i.pow(p);
                if c <= x {
                    max = cmp::max(max, c);
                }
                p += 1;
            }
        }
        println!("{}", max);
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
