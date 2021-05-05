use std::io;


fn main() {
    let n = read_one::<usize>();
    let mut v = read::<usize>();

    for i in 0..n {
        let el = v[i];
        let mut cur = i as isize - 1;
        while cur >= 0 && v[cur as usize] > el {
            v[cur as usize + 1] = v[cur as usize];
            cur -= 1;
        }
        v[(cur + 1) as usize] = el;

        for x in v.iter() {
            print!("{} ", x);
        }
        println!();
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
