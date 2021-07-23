fn main() {
    let (n, m) = {
        let i = io::read::<usize>();
        (i[0], i[1])
    };

    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        let (a, b) = {
            let i = io::read::<usize>();
            (i[0] - 1, i[1] - 1)
        };
        g[a].push(b);
        g[b].push(a);
    }

    let mut ans = 0;
    for i in 0..n {
        // total: O(M)
        let less = g[i].iter().filter(|&e| *e < i).count();
        if less == 1 { ans += 1; }
    }

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
