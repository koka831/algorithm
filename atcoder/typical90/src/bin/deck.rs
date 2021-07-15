use std::collections::VecDeque;

fn main() {
    let q = io::read_one::<usize>();
    let mut deq = VecDeque::new();

    for _ in 0..q {
        let (t, x) = {
            let i = io::read::<usize>();
            (i[0], i[1])
        };

        match t {
            1 => deq.push_front(x),
            2 => deq.push_back(x),
            3 => println!("{}", deq[x - 1]),
            _ => unreachable!(),
        }
    }
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
