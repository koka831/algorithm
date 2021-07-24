use itertools::Itertools;

fn main() {
    let n = io::read_one::<usize>();
    let mut ann = Vec::new();
    for _ in 0..n { ann.push(io::read::<usize>()); }

    let m = io::read_one::<usize>();
    let mut pass = vec![vec![true; n]; n];
    for _ in 0..m {
        let (x, y) = {
            let i = io::read::<usize>();
            (i[0] - 1, i[1] - 1)
        };

        pass[x][y] = false;
        pass[y][x] = false;
    }

    // player order
    let mut ans = std::usize::MAX;
    for pn in (0..n).permutations(n) {
        if (1..n).all(|i| pass[pn[i]][pn[i - 1]]) {
            let time = (0..n).map(|i| ann[pn[i]][i]).sum::<usize>();
            ans = ans.min(time);
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
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
