fn main() {
    let _n = io::read_one::<usize>();
    let an = io::read::<usize>();
    let bn = io::read::<usize>();
    let cn = io::read::<usize>();

    let mut a46 = vec![0; 46];
    let mut b46 = vec![0; 46];
    let mut c46 = vec![0; 46];

    for a in an { a46[a % 46] += 1; }
    for b in bn { b46[b % 46] += 1; }
    for c in cn { c46[c % 46] += 1; }

    let mut ans: usize = 0;
    for i in 0..46 { for j in 0..46 { for k in 0..46 {
        if (i + j + k) % 46 == 0 { ans += a46[i] * b46[j] * c46[k]; }
    }}}

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
