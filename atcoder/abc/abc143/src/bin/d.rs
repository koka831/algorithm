use std::io;
use std::cmp;


fn main() {
    let n = read_one::<usize>();
    let mut ln = read::<usize>();
    ln.sort();
    let mut ans: usize = 0;
    for i in 0..n - 1 { for j in (i + 1)..n - 1 {
        let pos = ln.lower_bound(&(ln[i] + ln[j]));
        ans += pos - j - 1;
    }}
    println!("{}", ans);
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

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => {
                    low = mid + 1;
                }
            }
        }
        low
    }
}
