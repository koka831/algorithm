use std::collections::HashMap;

fn main() {
    let (n, _m) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let mut hm = HashMap::new();
    for _ in 0..n {
        let s = io::read_one::<String>().chars().collect::<Vec<_>>();
        let mut zero = 0;
        let mut one = 0;
        for c in s {
            if c == '0' { zero += 1; }
            if c == '1' { one += 1; }
        }
        *hm.entry((zero, one)).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for &v in hm.values() {
        ans += v * (v - 1);
    }

    let mut dup: usize = 0;
    for &k in hm.keys() {
        dup += hm.get(&(k.1, k.0)).unwrap_or(&0);
    }

    println!("{}", ans + dup / 2);
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
