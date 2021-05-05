use std::io;

fn main() {
    let _n = read_one::<usize>();
    let a = read::<usize>();
    let mut cnt = 0;
    for i in a.into_iter() {
        let mut i_mut = i;
        loop {
           if i_mut % 2 == 0 {
            cnt += 1;
            i_mut = i_mut / 2;
           } else {
               break;
           }
        }
    }

    println!("{}", cnt);
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
