fn main() {
    let (n, mut h) = {
        let i = io::read::<usize>();
        (i[0], i[1] as isize)
    };

    let mut max = 0;
    let mut bn = Vec::new();
    for _ in 0..n {
        let (a, b) = {
            let i = io::read::<isize>();
            (i[0], i[1])
        };
        max = max.max(a);
        bn.push(b);
    }

    bn.sort_by_key(|x| -x);

    let mut cnt = 0;
    for b in bn {
        if h <= 0 || b <= max { break; }

        cnt += 1;
        h -= b;
    }

    // consider h < 0
    // if not, cnt may be decrement by negative throw
    if h > 0 { cnt += (h + max - 1) / max; }

    println!("{}", cnt);
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
