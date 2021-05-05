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

    let (a, b, c, x, y) = {
        let i = read::<usize>();
        (i[0], i[1], i[2], i[3], i[4])
    };

    if a + b < c * 2 {
        println!("{}", x * a + y * b);
    } else {
        let ab = cmp::min(x, y);
        let only = cmp::max(x, y) * c * 2;
        if x < y {
            println!("{}", cmp::min(b * (y - ab) + 2 * ab * c, only));
        } else {
            println!("{}", cmp::min(a * (x - ab) + 2 * ab * c, only));
        }
    }

}
