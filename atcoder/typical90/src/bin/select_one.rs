fn main() {
    let (n, k) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let an = io::read::<usize>();
    let bn = io::read::<usize>();

    let mut diff = 0;

    for i in 0..n {
        diff += an[i].max(bn[i]) - an[i].min(bn[i]);
    }

    if k >= diff && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
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
