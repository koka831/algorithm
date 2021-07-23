/// https://atcoder.jp/contests/typical90/tasks/typical90_n
fn main() {
    let n = io::read_one::<usize>();
    let mut an = io::read::<isize>();
    let mut bn = io::read::<isize>();
    an.sort();
    bn.sort();

    let ans = (0..n).map(|i| (an[i] - bn[i]).abs()).sum::<isize>();
    println!("{}", ans);
}

#[allow(unused)]
mod io {
    use std::io;
    pub fn read<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
    pub fn read_one<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
