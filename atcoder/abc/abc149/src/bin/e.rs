use std::io;
use std::cmp;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<usize>();
    an.sort();
    let mut l = 0;
    let mut r = *an.iter().max().unwrap() * 2 + 1;
    while r - l > 1 {
        let mid = (l + r) / 2;
        let mut c: usize = 0;
        for &a in an.iter() {
            let sub = mid as isize - a as isize;
            c += n - an.lower_bound(cmp::max(0, sub as usize))
        }
        if c >= m { l = mid; } else { r = mid; }
    }

    let mut acc = vec![0; n + 1];
    for i in 0..n { acc[i + 1] = acc[i] + an[i]; }

    let mut cnt = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        let j = an.upper_bound(
            cmp::max(0, (l as isize - an[i] as isize) as usize)
        );
        cnt += n - j;
        ans += acc[n] - acc[j] + an[i] * (n - j);
    }
    println!("{}", cnt);
    println!("{}", ans + (m - cnt) * l);
}

trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: T) -> usize {
        let mut l = -1isize;
        let mut r = self.len() as isize;
        while r - l > 1isize {
            let m = ((l + r) / 2) as usize;
            if self[m] < x { l = m as isize; }
            else { r = m as isize; }
        }
        r as usize
    }

    fn upper_bound(&self, x: T) -> usize {
        let mut l = -1isize;
        let mut r = self.len() as isize;

        while r - l > 1isize {
            let m = ((l + r) / 2) as usize;
            if self[m] <= x { l = m as isize; }
            else { r = m as isize; }
        }
        r as usize
    }
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
