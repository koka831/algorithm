use std::io;

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

fn main() {
    let (a, b, c) = {
        let v = read::<usize>();
        (v[0], v[1], v[2])
    };

    if a + b >= c {
        println!("Yes");
    } else {
        println!("No");
    }
}
