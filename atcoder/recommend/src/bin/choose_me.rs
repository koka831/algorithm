fn main() {
    let n = io::read_one::<usize>();
    let mut score = vec!();
    let mut sum = 0;
    for _ in 0..n {
        let (a, b) = {
            let i = io::read::<isize>();
            (i[0], i[1])
        };
        sum -= a;
        score.push(a * 2 + b);
    }

    score.sort_by_key(|x| -x);
    let mut ans = 0;
    for i in 0..n {
        if sum > 0 { break; }
        sum += score[i];
        ans += 1;
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
